#![forbid(unsafe_code)]

use flemish::{
    app,
    button::Button,
    color_themes,
    dialog::{choice2_default, FileChooser, FileChooserType},
    enums::{Shortcut,FrameType},
    frame::Frame,
    group::Flex,
    image::SharedImage,
    valuator::{Slider,SliderType},
    menu::{MenuButton, MenuFlag},
    prelude::*,
    OnEvent, OnMenuEvent, Sandbox, Settings,
};
use std::fs;

pub fn main() {
    Model::new().run(Settings {
        size: (640, 480),
        resizable: true,
        ignore_esc_close: true,
        color_map: Some(color_themes::DARK_THEME),
        scheme: Some(app::Scheme::Plastic),
        ..Default::default()
    })
}

const PAD: i32 = 10;
const HEIGHT: i32 = PAD * 3;

enum Page {
    Counter,
    Temperature,
}

struct Model {
    page: Page,
}

#[derive(Clone, Copy)]
enum Message {
    Counter,
    Temperature,
}

impl Sandbox for Model {
    type Message = Message;

    fn new() -> Self {
        Self { page: Page::Counter }
    }

    fn title(&self) -> String {
        String::from("FlSevenGUI")
    }

    fn view(&mut self) {
        let page = Flex::default_fill().column();
        match self.page {
            Page::Counter => {
                Frame::default().with_label("Counter");
                Button::default().with_label("Temperature").on_event(|_|Message::Temperature);
            }
            Page::Temperature => {
                Frame::default().with_label("Temperature");
                Button::default().with_label("Counter").on_event(|_|Message::Counter);
            }
        }
        page.end();
    }

    fn update(&mut self, message: Message) {
        self.page = match message {
            Message::Counter => Page::Counter,
            Message::Temperature => Page::Temperature,
        }
    }
}
