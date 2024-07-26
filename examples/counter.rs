#![forbid(unsafe_code)]

use flemish::{
    app, button::Button, color_themes, enums::FrameType, frame::Frame, group::Flex, prelude::*,
    OnEvent, Sandbox, Settings,
};

pub fn main() {
    Model::new().run(Settings {
        size: (640, 360),
        color_map: Some(color_themes::DARK_THEME),
        ..Default::default()
    })
}

#[derive(Debug, Clone, Copy)]
enum Message {
    Inc,
    Dec,
}

struct Model {
    value: u8,
}

impl Model {
    fn default() -> Self {
        Self { value: 0u8 }
    }
    fn inc(&mut self) {
        self.value = self.value.saturating_add(1);
    }
    fn dec(&mut self) {
        self.value = self.value.saturating_sub(1);
    }
    fn value(&self) -> String {
        self.value.to_string()
    }
}

impl Sandbox for Model {
    type Message = Message;

    fn new() -> Self {
        Self::default()
    }

    fn title(&self) -> String {
        format!("{} - 7GUI: Counter", self.value())
    }

    fn view(&mut self) {
        let mut page = Flex::default().with_size(300, 100).center_of_parent();
        {
            Button::default()
                .with_label("@#<")
                .on_event(|_| Message::Dec);
            Frame::default()
                .with_label(&self.value.to_string())
                .set_frame(FrameType::UpBox);
            Button::default()
                .with_label("@#>")
                .on_event(|_| Message::Inc);
        }
        page.end();
        page.set_pad(0);
        page.set_margin(10);
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::Inc => self.inc(),
            Message::Dec => self.dec(),
        }
    }
}
