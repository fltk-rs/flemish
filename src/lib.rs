/*!
# Flemish

An elmish architecture for fltk-rs.

## Usage
Add flemish to your dependencies:
```toml,ignore
[dependencies]
flemish = "0.1"
```

A usage example:
```rust,no_run
use flemish::{
    button::Button, frame::Frame, Flex, FlexType, OnEvent, Sandbox, Settings
};

pub fn main() {
    Counter::new().run(Settings {
        size: (300, 100),
        resizable: true,
        ..Default::default()
    })
}

#[derive(Default)]
struct Counter {
    value: i32,
}

#[derive(Debug, Clone, Copy)]
enum Message {
    IncrementPressed,
    DecrementPressed,
}

impl OnEvent<Message> for Button {}

impl Sandbox for Counter {
    type Message = Message;

    fn new() -> Self {
        Self::default()
    }

    fn title(&self) -> String {
        String::from("Counter - fltk-rs")
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::IncrementPressed => {
                self.value += 1;
            }
            Message::DecrementPressed => {
                self.value -= 1;
            }
        }
    }

    fn view(&mut self) -> Flex {
        let mut col = Flex::default().with_type(FlexType::Column);
        let mut button1 = Button::default().with_label("Increment");
        button1.on_event(Message::IncrementPressed);
        Frame::default().with_label(&self.value.to_string());
        let mut button2 = Button::default().with_label("Decrement");
        button2.on_event(Message::DecrementPressed);
        col.end();
        col
    }
}
```
*/

pub use fltk::*;
pub use fltk_flex::*;
use fltk::prelude::*;

pub trait OnEvent<T>: WidgetExt
where
    T: Send + Sync + Clone + 'static,
{
    fn on_event(&mut self, msg: T)
    where
        Self: Sized,
    {
        let (s, _) = app::channel::<T>();
        self.emit(s, msg);
    }
}

#[derive(Default)]
pub struct Settings {
    pub size: (i32, i32),
    pub resizable: bool,
    pub color: Option<enums::Color>,
}

pub trait Sandbox {
    type Message: Clone + Send + Sync;
    fn new() -> Self;
    fn title(&self) -> String;
    fn view(&mut self) -> Flex;
    fn update(&mut self, message: Self::Message);
    fn run(&mut self, settings: Settings) {
        let a = app::App::default().with_scheme(app::Scheme::Gtk);
        app::get_system_colors();
        let (w, h) = settings.size;
        let w = if w == 0 { 400 } else { w };
        let h = if h == 0 { 300 } else { h };
        let mut win = window::Window::default()
            .with_size(w, h)
            .with_label(&self.title());
        let mut grp = group::Group::default().size_of_parent();
        self.view();
        grp.end();
        win.end();
        win.make_resizable(settings.resizable);
        win.show();
        let (_, r) = app::channel::<Self::Message>();
        while a.wait() {
            if let Some(msg) = r.recv() {
                self.update(msg);
                grp.remove_by_index(0);
                let mut v = self.view();
                grp.add(&*v);
                v.resize(grp.x(), grp.y(), grp.w(), grp.h());
                app::redraw();
            }
        }
    }
}
