use fltk::app::Sender;
use fltk::enums::Event;
use std::sync::{
    atomic::{AtomicBool, AtomicI32, Ordering},
    Arc,
};
use std::time::{Duration, Instant};

pub enum SubType<M> {
    None,
    Instant(Box<dyn Fn(Instant) -> Option<M> + Send + Sync>),
    Event(Box<dyn Fn(Event) -> Option<M> + Send + Sync>),
}

pub struct Subscription<M> {
    pub interval: Option<u64>,
    pub generator: SubType<M>,
    pub cancel_flag: Option<Arc<AtomicBool>>,
}

impl Subscription<Instant> {
    pub fn every(duration: Duration) -> Self {
        Subscription::run(duration, |_| Instant::now())
    }

    pub fn run<F>(duration: Duration, f: F) -> Self
    where
        F: 'static + Send + Sync + Fn(Duration) -> Instant,
    {
        Subscription {
            interval: Some(duration.as_millis() as u64),
            generator: SubType::Instant(Box::new(move |start| Some(f(start.elapsed())))),
            cancel_flag: None,
        }
    }
}

impl Subscription<Event> {
    pub fn events() -> Self {
        Subscription {
            interval: Some(8),
            generator: SubType::Event(Box::new(Some)),
            cancel_flag: None,
        }
    }

    pub fn on_event<F>(filter: F) -> Subscription<Event>
    where
        F: Fn(Event) -> bool + Send + Sync + 'static,
    {
        Subscription {
            interval: None,
            generator: SubType::Event(Box::new(
                move |ev: Event| {
                    if filter(ev) {
                        Some(ev)
                    } else {
                        None
                    }
                },
            )),
            cancel_flag: None,
        }
    }
}

impl<M> Subscription<M>
where
    M: Clone + Send + Sync + 'static,
{
    pub fn none() -> Self {
        Subscription {
            interval: None,
            generator: SubType::None,
            cancel_flag: None,
        }
    }

    pub fn map<U, F>(self, f: F) -> Subscription<U>
    where
        U: Clone + Send + Sync + 'static,
        F: Fn(M) -> U + Send + Sync + 'static + Clone,
    {
        match self.generator {
            SubType::None => Subscription::none(),
            SubType::Instant(g) => Subscription {
                interval: self.interval,
                cancel_flag: self.cancel_flag.clone(),
                generator: SubType::Instant(Box::new(move |now| g(now).map(&f))),
            },
            SubType::Event(g) => Subscription {
                interval: self.interval,
                cancel_flag: self.cancel_flag.clone(),
                generator: SubType::Event(Box::new(move |ev| g(ev).map(&f))),
            },
        }
    }

    pub fn cancelable(self, flag: Arc<AtomicBool>) -> Self {
        Subscription {
            cancel_flag: Some(flag),
            ..self
        }
    }
}

pub fn batch<M>(subs: Vec<Subscription<M>>) -> Vec<Subscription<M>> {
    subs
}

pub fn start_subscription<M>(
    win: &mut fltk::window::Window,
    subscriptions: Vec<Subscription<M>>,
    sender: Sender<M>,
) where
    M: Clone + Send + Sync + 'static,
{
    use fltk::prelude::WidgetBase;
    use tokio::task;

    let last_event = Arc::new(AtomicI32::new(0));
    let event = Arc::new(AtomicI32::new(0));

    win.handle({
        let event = event.clone();
        move |_w, ev| {
            if ev != Event::NoEvent {
                event.store(ev.bits(), Ordering::Relaxed);
            }
            false
        }
    });

    for sub in subscriptions {
        let sender = sender.clone();
        let cancel_flag = sub.cancel_flag.clone();

        let interval = sub.interval.unwrap_or(16);

        match sub.generator {
            SubType::None => {}
            SubType::Instant(f) => {
                task::spawn(async move {
                    loop {
                        if let Some(flag) = &cancel_flag {
                            if flag.load(Ordering::Relaxed) {
                                break;
                            }
                        }
                        if interval > 0 {
                            tokio::time::sleep(Duration::from_millis(interval)).await;
                        }
                        let now = Instant::now();
                        if let Some(msg) = f(now) {
                            sender.send(msg);
                        }
                    }
                });
            }
            SubType::Event(f) => {
                let last_event = last_event.clone();
                let event = event.clone();
                task::spawn(async move {
                    loop {
                        if let Some(flag) = &cancel_flag {
                            if flag.load(Ordering::Relaxed) {
                                break;
                            }
                        }
                        if interval > 0 {
                            tokio::time::sleep(Duration::from_millis(interval)).await;
                        }

                        if event.load(Ordering::Relaxed) != last_event.load(Ordering::Relaxed) {
                            last_event.store(event.load(Ordering::Relaxed), Ordering::Relaxed);
                            if let Some(msg) = f(Event::from_i32(event.load(Ordering::Relaxed))) {
                                sender.send(msg);
                            }
                        }
                    }
                });
            }
        }
    }
}
