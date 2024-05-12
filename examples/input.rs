use flemish::{
    app, button::Button, color_themes, frame::Frame, group::Flex, input::Input, prelude::*,
    OnEvent, Sandbox, Settings,
};

pub fn main() {
    State::new().run(Settings {
        size: (300, 100),
        resizable: true,
        color_map: Some(color_themes::DARK_THEME),
        scheme: Some(app::Scheme::Base),
        ..Default::default()
    })
}

#[derive(Default)]
struct State {
    text: String,
}

#[derive(Debug, Clone)]
enum Message {
    Submit(String),
}

impl Sandbox for State {
    type Message = Message;

    fn new() -> Self {
        Self::default()
    }

    fn title(&self) -> String {
        String::from("State - fltk-rs")
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::Submit(value) => {
                self.text = value.clone();
                println!("Hello {}", value);
            }
        }
    }

    fn view(&mut self) {
        let col = Flex::default_fill().column();
        Frame::default().with_label("Enter name:");
        let mut name = Input::default();
        name.set_value(&self.text);
        Button::default()
            .with_label("Submit")
            .on_event(move |_| Message::Submit(name.value()));
        col.end();
    }
}
