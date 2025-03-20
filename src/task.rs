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
        }
    }

    pub fn exit() -> Self {
        Self {
            executor: Executor::Exit,
        }
    }

    pub fn perform_simple(func: fn() -> M) -> Self {
        Self {
            executor: Executor::SyncFn(func),
        }
    }

    pub fn perform<F>(func: F) -> Self
    where
        F: FnOnce() -> M + Send + 'static,
    {
        Self {
            executor: Executor::SyncClosure(Box::new(func)),
        }
    }

    pub fn perform_async<Fut, F>(future: F) -> Self
    where
        Fut: Future<Output = M> + Send + 'static,
        F: FnOnce() -> Fut + Send + 'static,
    {
        Self {
            executor: Executor::Async(Box::new(move || Box::pin(future()))),
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
            },
            Executor::Exit => Task {
                executor: Executor::Exit,
            },
            Executor::SyncFn(func) => Task::perform(move || mapper(func())),
            Executor::SyncClosure(func) => Task::perform(move || mapper(func())),
            Executor::Async(fut) => Task::perform_async(move || async move {
                let result = fut().await;
                mapper(result)
            }),
        }
    }

    pub fn execute(self, sender: Sender<M>) {
        match self.executor {
            Executor::None => {}
            Executor::Exit => {
                fltk::app::quit();
            }
            Executor::SyncFn(func) => {
                std::thread::spawn(move || {
                    sender.send(func());
                });
            }
            Executor::SyncClosure(func) => {
                std::thread::spawn(move || {
                    sender.send(func());
                });
            }
            Executor::Async(fut) => {
                task::spawn(async move {
                    let val = fut().await;
                    sender.send(val);
                });
            }
        }
    }

    pub fn cancelable(self, flag: Arc<AtomicBool>) -> Self {
        match self.executor {
            Executor::None => self,
            Executor::Exit => self,
            Executor::SyncFn(func) => Task::perform(move || {
                if flag.load(Ordering::Relaxed) {
                    func()
                } else {
                    panic!("[Task] Canceled execution.")
                }
            }),
            Executor::SyncClosure(func) => Task::perform(move || {
                if flag.load(Ordering::Relaxed) {
                    func()
                } else {
                    panic!("[Task] Canceled execution.")
                }
            }),
            Executor::Async(fut) => Task::perform_async(move || async move {
                if flag.load(Ordering::Relaxed) {
                    fut().await
                } else {
                    panic!("[Task] Canceled async execution.")
                }
            }),
        }
    }
}

pub fn batch<M>(tasks: Vec<Task<M>>, sender: Sender<M>)
where
    M: Send + Sync + 'static,
{
    for task in tasks {
        task.execute(sender.clone());
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
            results.push(rx.recv().unwrap());
        }
        results
    })
}
