#![forbid(unsafe_code)]

use flemish::{
    app, color_themes, button::Button, frame::Frame, group::Flex, prelude::*, OnEvent, Sandbox, Settings, valuator::Dial,
};

pub fn main() {
    Counter::new().run(Settings {
        size: (100, 300),
        resizable: false,
        ignore_esc_close: true,
        color_map: Some(color_themes::DARK_THEME),
        scheme: Some(app::Scheme::Base),
        ..Default::default()
    })
}

const PAD: i32 = 10;
const DIAL: u8 = 120;

struct Counter {
    value: u8,
}

#[derive(Debug, Clone, Copy)]
enum Message {
    IncrementPressed,
    DecrementPressed,
}

impl Sandbox for Counter {
    type Message = Message;

    fn new() -> Self {
        Self {value: 0}
    }

    fn title(&self) -> String {
        String::from("Counter - fltk-rs")
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::IncrementPressed => {
                if self.value == DIAL - 1 {
                    self.value = 0;
                } else {
                    self.value += 1;
                }
            }
            Message::DecrementPressed => {
                if self.value == 0 {
                    self.value = DIAL - 1;
                } else {
                    self.value -= 1;
                }
            }
        }
    }

    fn view(&mut self) {
        let mut page = Flex::default_fill().column();
        Button::default()
            .with_label("Increment")
            .on_event(|_|Message::IncrementPressed);
        Frame::default().with_label(&self.value.to_string());
        Button::default()
            .with_label("Decrement")
            .on_event(|_|Message::DecrementPressed);
        let mut dial = Dial::default();
        dial.set_maximum((DIAL / 4 * 3) as f64);
        dial.set_value(self.value as f64);
        dial.deactivate();
        page.end();
        page.set_margin(PAD);
    }
}
