#![forbid(unsafe_code)]

use flemish::{
    app, button::CheckButton, color_themes, group::Flex, prelude::*,
    OnEvent, Sandbox, Settings,
};

pub fn main() {
    Model::new().run(Settings {
        size: (300, 300),
        resizable: false,
        ignore_esc_close: true,
        color_map: Some(color_themes::DARK_THEME),
        scheme: Some(app::Scheme::Base),
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
    fn new() -> Self {
        Self {
            default: true,
            styled: false,
            custom: false,
        }
    }

    fn view(&mut self) {
        let mut page = Flex::default_fill().column();
        {
            crate::check(self.default).on_event(move |check| Message::DefaultToggled(check.value()));
            crate::check(self.styled).on_event(move |check| Message::StyledToggled(check.value()));
            crate::check(self.custom).on_event(move |check| Message::CustomToggled(check.value()));
        }
        page.end();
        page.set_margin(PAD);
    }

    type Message = Message;
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

    fn title(&self) -> String {
        String::from("CheckButton - Flemish")
    }
}

fn check(value: bool) -> CheckButton {
    let mut element = CheckButton::default();
    element.set_value(value);
    element
}
