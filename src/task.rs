use fltk::app::Sender;
use std::future::Future;
use std::{
    pin::Pin,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
};
use tokio::task;

pub struct Task<M: Send + Sync> {
    executor: Executor<M>,
    cancel_flag: Option<Arc<AtomicBool>>,
}

impl<Message: Send + Sync + 'static> From<()> for Task<Message> {
    fn from(_: ()) -> Self {
        Task::none()
    }
}

impl<Message: Send + Sync + 'static, E> From<Result<Task<Message>, E>> for Task<Message> {
    fn from(s: Result<Task<Message>, E>) -> Self {
        if let Ok(s) = s {
            s
        } else {
            Task::none()
        }
    }
}

enum Executor<M: Send + Sync> {
    None,
    Exit,
    SyncFn(fn() -> M),
    SyncClosure(Box<dyn FnOnce() -> M + Send>),
    Async(Box<dyn FnOnce() -> Pin<Box<dyn Future<Output = M> + Send + 'static>> + Send>),
}

impl<M> Task<M>
where
    M: Send + Sync + 'static,
{
    pub fn none() -> Self {
        Self {
            executor: Executor::None,
            cancel_flag: None,
        }
    }

    pub fn exit() -> Self {
        Self {
            executor: Executor::Exit,
            cancel_flag: None,
        }
    }

    pub fn perform_simple(func: fn() -> M) -> Self {
        Self {
            executor: Executor::SyncFn(func),
            cancel_flag: None,
        }
    }

    pub fn perform<F>(func: F) -> Self
    where
        F: FnOnce() -> M + Send + 'static,
    {
        Self {
            executor: Executor::SyncClosure(Box::new(func)),
            cancel_flag: None,
        }
    }

    pub fn perform_async<Fut, F>(future: F) -> Self
    where
        Fut: Future<Output = M> + Send + 'static,
        F: FnOnce() -> Fut + Send + 'static,
    {
        Self {
            executor: Executor::Async(Box::new(move || Box::pin(future()))),
            cancel_flag: None,
        }
    }

    pub fn map<N, F>(self, mapper: F) -> Task<N>
    where
        N: Send + Sync + 'static,
        F: FnOnce(M) -> N + Send + 'static,
    {
        match self.executor {
            Executor::None => Task {
                executor: Executor::None,
                cancel_flag: self.cancel_flag,
            },
            Executor::Exit => Task {
                executor: Executor::Exit,
                cancel_flag: self.cancel_flag,
            },
            Executor::SyncFn(func) => Task {
                executor: Executor::SyncClosure(Box::new(move || mapper(func()))),
                cancel_flag: self.cancel_flag,
            },
            Executor::SyncClosure(func) => Task {
                executor: Executor::SyncClosure(Box::new(move || mapper(func()))),
                cancel_flag: self.cancel_flag,
            },
            Executor::Async(fut) => Task {
                executor: Executor::Async(Box::new(move || {
                    Box::pin(async move {
                        let result = fut().await;
                        mapper(result)
                    })
                })),
                cancel_flag: self.cancel_flag,
            },
        }
    }

    pub fn execute(self, sender: Sender<M>) {
        let canceled = self.cancel_flag.clone();
        match self.executor {
            Executor::None => {}
            Executor::Exit => {
                fltk::app::quit();
            }
            Executor::SyncFn(func) => {
                std::thread::spawn(move || {
                    let val = func();
                    if canceled
                        .as_ref()
                        .map(|f| !f.load(Ordering::Relaxed))
                        .unwrap_or(true)
                    {
                        sender.send(val);
                    }
                });
            }
            Executor::SyncClosure(func) => {
                std::thread::spawn(move || {
                    let val = func();
                    if canceled
                        .as_ref()
                        .map(|f| !f.load(Ordering::Relaxed))
                        .unwrap_or(true)
                    {
                        sender.send(val);
                    }
                });
            }
            Executor::Async(fut) => {
                task::spawn(async move {
                    let val = fut().await;
                    if canceled
                        .as_ref()
                        .map(|f| !f.load(Ordering::Relaxed))
                        .unwrap_or(true)
                    {
                        sender.send(val);
                    }
                });
            }
        }
    }

    pub fn cancelable(mut self, flag: Arc<AtomicBool>) -> Self {
        self.cancel_flag = Some(flag);
        self
    }
}

pub fn batch<M>(tasks: Vec<Task<M>>, sender: Sender<M>)
where
    M: Send + Sync + 'static,
{
    for task in tasks {
        task.execute(sender);
    }
}

pub fn join<M>(tasks: Vec<Task<M>>) -> Task<Vec<M>>
where
    M: Send + Sync + 'static,
{
    Task::perform_async(move || async move {
        let mut results = Vec::with_capacity(tasks.len());
        for t in tasks {
            let (tx, rx) = fltk::app::channel::<M>();
            t.execute(tx);
            if let Some(msg) = rx.recv() {
                results.push(msg);
            }
            // If the task was canceled and sent nothing, skip to avoid panics.
        }
        results
    })
}
