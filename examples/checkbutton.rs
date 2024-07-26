#![forbid(unsafe_code)]

use flemish::{
    app, button::CheckButton, color_themes, enums::FrameType, group::Flex, prelude::*, OnEvent,
    Sandbox, Settings,
};

pub fn main() {
    Model::new().run(Settings {
        size: (640, 360),
        color_map: Some(color_themes::DARK_THEME),
        ..Default::default()
    })
}

const PAD: i32 = 10;

struct Model {
    default: bool,
    styled: bool,
    custom: bool,
}

#[derive(Clone, Copy)]
enum Message {
    DefaultToggled(bool),
    CustomToggled(bool),
    StyledToggled(bool),
}

impl Sandbox for Model {
    type Message = Message;

    fn new() -> Self {
        Self {
            default: true,
            styled: false,
            custom: false,
        }
    }

    fn title(&self) -> String {
        String::from("CheckButton - Flemish")
    }

    fn view(&mut self) {
        let mut page = Flex::default()
            .with_size(300, 150)
            .center_of_parent()
            .column();
        {
            crate::check(self.default)
                .on_event(move |check| Message::DefaultToggled(check.value()));
            crate::check(self.styled).on_event(move |check| Message::StyledToggled(check.value()));
            crate::check(self.custom).on_event(move |check| Message::CustomToggled(check.value()));
        }
        page.end();
        page.set_pad(PAD);
        page.set_frame(FrameType::UpBox);
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::DefaultToggled(value) => {
                self.default = value;
            }
            Message::StyledToggled(value) => {
                self.styled = value;
            }
            Message::CustomToggled(value) => {
                self.custom = value;
            }
        }
    }
}

fn check(value: bool) -> CheckButton {
    let mut element = CheckButton::default();
    element.set_value(value);
    element
}
