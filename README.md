# Flemish

An elmish architecture for fltk-rs, inspired by Iced.

## Usage
Add flemish to your dependencies:
```toml
[dependencies]
flemish = "0.2"
```

A usage example:
```rust
use flemish::{
    color_themes, button::Button, frame::Frame, group::Flex, prelude::*, OnEvent, Sandbox, Settings,
};

pub fn main() {
    Counter::new().run(Settings {
        size: (300, 100),
        resizable: true,
        color_map: Some(color_themes::BLACK_THEME),
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

    fn view(&mut self) {
        let col = Flex::default_fill().column();
        let mut button1 = Button::default().with_label("Increment");
        button1.on_event(Message::IncrementPressed);
        Frame::default().with_label(&self.value.to_string());
        let mut button2 = Button::default().with_label("Decrement");
        button2.on_event(Message::DecrementPressed);
        col.end();
    }
}
```
