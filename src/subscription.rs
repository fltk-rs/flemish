use async_stream::stream;
use fltk::{app::Sender, enums::Event};
use futures::{stream::BoxStream, StreamExt};
use fxhash::FxHasher;
use std::{
    any::TypeId,
    hash::{Hash, Hasher},
    marker::PhantomData,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc, OnceLock,
    },
    time::{Duration, Instant},
};
use tokio::{
    sync::mpsc::{unbounded_channel, UnboundedSender},
    task,
};
use tokio_stream::wrappers::UnboundedReceiverStream;

pub(crate) static EVENTS_CONTEXT: OnceLock<Option<EventsContext>> = OnceLock::new();

pub(crate) struct EventsContext {
    pub(crate) last_event: Arc<std::sync::atomic::AtomicI32>,
    pub(crate) current_event: Arc<std::sync::atomic::AtomicI32>,
}

pub trait Recipe {
    type Output: Clone + Send + Sync + 'static;

    fn stream(self: Box<Self>) -> BoxStream<'static, Self::Output>;

    fn hash(&self, state: &mut FxHasher);
}

pub enum Subscription<M>
where
    M: Send + Sync + 'static,
{
    None,
    Recipe {
        recipe: Box<dyn Recipe<Output = M> + Send + Sync>,
        cancel_flag: Option<Arc<AtomicBool>>,
    },
}

impl<M> Subscription<M>
where
    M: Clone + Send + Sync + 'static,
{
    pub fn none() -> Self {
        Subscription::None
    }

    pub fn from_recipe<R>(recipe: R) -> Self
    where
        R: Recipe<Output = M> + Send + Sync + 'static,
    {
        Subscription::Recipe {
            recipe: Box::new(recipe),
            cancel_flag: None,
        }
    }

    pub fn run<F>(f: F) -> Self
    where
        F: FnOnce(UnboundedSender<M>) + Send + Sync + 'static,
    {
        Subscription::from_recipe(GenericSyncRecipe::new(f))
    }

    pub fn run_async<F, Fut>(f: F) -> Self
    where
        F: FnOnce(UnboundedSender<M>) -> Fut + Send + Sync + 'static,
        Fut: std::future::Future<Output = ()> + Send + 'static,
    {
        Subscription::from_recipe(GenericAsyncRecipe::new(f))
    }

    pub fn cancelable(mut self, flag: Arc<AtomicBool>) -> Self {
        if let Subscription::Recipe { cancel_flag, .. } = &mut self {
            *cancel_flag = Some(flag);
        }
        self
    }

    pub fn map<N, F>(self, f: F) -> Subscription<N>
    where
        N: Clone + Send + Sync + 'static,
        F: Fn(M) -> N + Send + Sync + 'static + Clone,
    {
        match self {
            Subscription::None => Subscription::None,
            Subscription::Recipe {
                recipe,
                cancel_flag,
            } => Subscription::Recipe {
                recipe: Box::new(MapRecipe {
                    inner: recipe,
                    mapper: f,
                }),
                cancel_flag,
            },
        }
    }
}

impl Subscription<Instant> {
    pub fn every(duration: Duration) -> Subscription<Instant> {
        Subscription::from_recipe(EveryRecipe { duration })
    }
}

impl Subscription<Event> {
    pub fn events() -> Subscription<Event> {
        Subscription::from_recipe(EventsRecipe {
            interval: Duration::from_millis(8),
        })
    }
}

pub fn batch<M: Send + Sync>(subs: Vec<Subscription<M>>) -> Vec<Subscription<M>> {
    subs
}

pub(crate) fn spawn_new_subscription<M>(sub: Subscription<M>, sender: Sender<M>) -> Subscription<M>
where
    M: Clone + Send + Sync + 'static,
{
    match sub {
        Subscription::None => Subscription::None,
        Subscription::Recipe {
            recipe,
            cancel_flag,
        } => {
            let local_cancel = cancel_flag.clone();

            let mut stream = recipe.stream();

            task::spawn(async move {
                while let Some(msg) = stream.next().await {
                    if let Some(cf) = &local_cancel {
                        if cf.load(std::sync::atomic::Ordering::Relaxed) {
                            break;
                        }
                    }
                    sender.send(msg.clone());
                }
            });

            Subscription::None
        }
    }
}

pub(crate) fn cancel_subscription<M>(sub: Option<Subscription<M>>)
where
    M: Clone + Send + Sync + 'static,
{
    if let Some(Subscription::Recipe {
        cancel_flag: Some(cf),
        ..
    }) = sub
    {
        cf.store(true, std::sync::atomic::Ordering::Relaxed);
    }
}

pub(crate) fn spawn_or_reuse_subscription<M>(sub: &Subscription<M>) -> u64
where
    M: Clone + Send + Sync + 'static,
{
    match sub {
        Subscription::None => 0,
        Subscription::Recipe { recipe, .. } => {
            let mut hasher = FxHasher::default();
            recipe.hash(&mut hasher);
            hasher.finish()
        }
    }
}

struct EveryRecipe {
    pub duration: Duration,
}

impl Recipe for EveryRecipe {
    type Output = Instant;

    fn stream(self: Box<Self>) -> BoxStream<'static, Instant> {
        let duration = self.duration;
        let s = stream! {
            loop {
                tokio::time::sleep(duration).await;
                yield Instant::now();
            }
        };
        s.boxed()
    }

    fn hash(&self, state: &mut FxHasher) {
        TypeId::of::<Self>().hash(state);
        let nanos = self.duration.as_nanos() as u64;
        nanos.hash(state);
    }
}

struct EventsRecipe {
    pub interval: Duration,
}

impl Recipe for EventsRecipe {
    type Output = Event;

    fn stream(self: Box<Self>) -> BoxStream<'static, Event> {
        let interval = self.interval;
        let s = stream! {
            if let Some(Some(context)) = EVENTS_CONTEXT.get() {
                let last_event = &context.last_event;
                let current_event = &context.current_event;
                loop {
                    tokio::time::sleep(interval).await;
                    let c = current_event.load(Ordering::Relaxed);
                    let l = last_event.load(Ordering::Relaxed);
                    if c != l {
                        last_event.store(c, Ordering::Relaxed);
                        let ev = Event::from_i32(c);
                        if ev != Event::NoEvent {
                            yield ev;
                        }
                    }
                }
            }
        };
        s.boxed()
    }

    fn hash(&self, state: &mut FxHasher) {
        TypeId::of::<Self>().hash(state);
        let nanos = self.interval.as_nanos() as u64;
        nanos.hash(state);
    }
}

struct MapRecipe<In, Out, F>
where
    F: Fn(In) -> Out + Clone + Send + Sync + 'static,
    In: Clone + Send + Sync + 'static,
    Out: Clone + Send + Sync + 'static,
{
    inner: Box<dyn Recipe<Output = In> + Send + Sync>,
    mapper: F,
}

impl<In, Out, F> Recipe for MapRecipe<In, Out, F>
where
    F: Fn(In) -> Out + Clone + Send + Sync + 'static,
    In: Clone + Send + Sync + 'static,
    Out: Clone + Send + Sync + 'static,
{
    type Output = Out;

    fn stream(self: Box<Self>) -> BoxStream<'static, Out> {
        let mapper = self.mapper.clone();
        let mut inner_stream = self.inner.stream();
        let s = stream! {
            while let Some(value_in) = inner_stream.next().await {
                yield mapper(value_in);
            }
        };
        s.boxed()
    }

    fn hash(&self, state: &mut FxHasher) {
        self.inner.hash(state);
        TypeId::of::<F>().hash(state);
    }
}

pub struct GenericAsyncRecipe<M, F> {
    f: Option<F>,
    _marker: PhantomData<fn() -> M>,
}

impl<M, F, Fut> GenericAsyncRecipe<M, F>
where
    M: Clone + Send + Sync + 'static,
    F: FnOnce(UnboundedSender<M>) -> Fut + Send + Sync + 'static,
    Fut: std::future::Future<Output = ()> + Send + 'static,
{
    pub fn new(f: F) -> Self {
        Self {
            f: Some(f),
            _marker: PhantomData,
        }
    }
}

impl<M, F, Fut> Recipe for GenericAsyncRecipe<M, F>
where
    M: Clone + Send + Sync + 'static,
    F: FnOnce(UnboundedSender<M>) -> Fut + Send + Sync + 'static,
    Fut: std::future::Future<Output = ()> + Send + 'static,
{
    type Output = M;

    fn stream(self: Box<Self>) -> BoxStream<'static, M> {
        let (tx, rx) = unbounded_channel::<M>();
        let mut f_opt = self.f;
        let s = stream! {
            if let Some(f) = f_opt.take() {
                tokio::task::spawn(async move {
                    f(tx).await;
                });
            }
            let mut rx_stream = UnboundedReceiverStream::new(rx);
            while let Some(msg) = rx_stream.next().await {
                yield msg;
            }
        };
        s.boxed()
    }

    fn hash(&self, state: &mut FxHasher) {
        TypeId::of::<F>().hash(state);
    }
}

pub struct GenericSyncRecipe<M, F> {
    f: Option<F>,
    _marker: PhantomData<fn() -> M>,
}

impl<M, F> GenericSyncRecipe<M, F>
where
    M: Clone + Send + Sync + 'static,
    F: FnOnce(UnboundedSender<M>) + Send + 'static,
{
    pub fn new(f: F) -> Self {
        Self {
            f: Some(f),
            _marker: PhantomData,
        }
    }
}

impl<M, F> Recipe for GenericSyncRecipe<M, F>
where
    M: Clone + Send + Sync + 'static,
    F: FnOnce(UnboundedSender<M>) + Send + 'static,
{
    type Output = M;

    fn stream(self: Box<Self>) -> BoxStream<'static, M> {
        let (tx, rx) = unbounded_channel::<M>();
        let mut f_opt = self.f;
        let s = stream! {
            if let Some(f) = f_opt.take() {
                tokio::task::spawn_blocking(move || {
                    f(tx);
                });
            }
            let mut rx_stream = UnboundedReceiverStream::new(rx);
            while let Some(msg) = rx_stream.next().await {
                yield msg;
            }
        };
        s.boxed()
    }

    fn hash(&self, state: &mut FxHasher) {
        TypeId::of::<F>().hash(state);
    }
}
