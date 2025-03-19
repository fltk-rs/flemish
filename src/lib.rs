#![doc = include_str!("../README.md")]
#![allow(clippy::needless_doctest_main)]

mod application;
pub mod dialog;
pub mod enums;
mod id;
pub mod props;
mod settings;
pub mod subscription;
pub mod task;
pub mod theme;
pub mod utils;
pub mod vdom;
pub mod view;
pub mod vnode;
pub mod widgets;

use crate::application::Application;
pub use fltk::app::Scheme;
pub use settings::Settings;
pub use subscription::Subscription;
pub use task::Task;
pub use view::View;

pub fn run<T: Default + 'static, Message: Clone + Send + Sync + 'static, U: Into<Task<Message>>>(
    name: &str,
    update_fn: fn(&mut T, Message) -> U,
    view_fn: fn(&T) -> View<Message>,
) {
    let a = Application::new(name, update_fn, view_fn);
    a.run();
}

pub fn application<T: 'static, Message: Clone + Send + Sync + 'static, U: Into<Task<Message>>>(
    name: &str,
    update_fn: fn(&mut T, Message) -> U,
    view_fn: fn(&T) -> View<Message>,
) -> Application<T, Message, U> {
    Application::new(name, update_fn, view_fn)
}
