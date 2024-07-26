#![forbid(unsafe_code)]

mod model;

use {
    flemish::{
        button::Button, color_themes, dialog::alert_default, enums::FrameType, frame::Frame,
        group::Flex, input::Input, menu::Choice, prelude::*, OnEvent, Sandbox, Settings,
    },
    model::Model,
};

pub fn main() {
    Model::new().run(Settings {
        size: (640, 360),
        color_map: Some(color_themes::DARK_THEME),
        ..Default::default()
    })
}

const NAME: &str = "7GUI: Flightbooker";
const PAD: i32 = 10;
const HEIGHT: i32 = PAD * 3;
const WIDTH: i32 = PAD * 12;

#[derive(Clone)]
pub enum Message {
    Direct(i32),
    Start(String),
    Back(String),
    Book,
}

impl Sandbox for Model {
    type Message = Message;

    fn title(&self) -> String {
        String::from(NAME)
    }

    fn new() -> Self {
        Self::default()
    }

    fn view(&mut self) {
        let mut page = Flex::default()
            .with_size(PAD * 26, PAD * 17)
            .center_of_parent();
        {
            page.fixed(&Frame::default(), WIDTH);
            let mut right = Flex::default().column();
            crate::choice(self.direct, &mut right)
                .with_label("Direct")
                .on_event(move |choice| Message::Direct(choice.value()));
            crate::input(&self.start, self.start_active)
                .with_label("Start")
                .on_event(move |input| Message::Start(input.value()));
            crate::input(&self.back, self.back_active)
                .with_label("Back")
                .on_event(move |input| Message::Back(input.value()));
            crate::button(self.book_active, &mut right)
                .with_label("Book")
                .clone()
                .on_event(move |_| Message::Book);
            right.end();
            right.set_pad(PAD);
            page.end();
            page.set_pad(0);
            page.set_margin(PAD);
            page.set_frame(FrameType::UpBox);
        }
        page.set_margin(PAD);
        page.set_pad(PAD);
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::Direct(value) => self.direct(value),
            Message::Start(value) => self.start(value),
            Message::Back(value) => self.back(value),
            Message::Book => {
                alert_default(&if self.direct == 0 {
                    format!("You have booked a one-way flight for {}.", self.start)
                } else {
                    format!(
                        "You have booked a return flight from {} to {}",
                        self.start, self.back
                    )
                });
            }
        }
    }
}

fn input(value: &str, active: bool) -> Input {
    let mut element = Input::default();
    element.set_value(value);
    if active {
        element.activate();
    } else {
        element.deactivate();
    };
    element
}

fn choice(value: i32, flex: &mut Flex) -> Choice {
    let mut element = Choice::default();
    element.add_choice("one-way flight|return flight");
    element.set_value(value);
    flex.fixed(&element, HEIGHT);
    element
}

fn button(active: bool, flex: &mut Flex) -> Button {
    let mut element = Button::default();
    if active {
        element.activate();
    } else {
        element.deactivate();
    };
    flex.fixed(&element, HEIGHT);
    element
}
