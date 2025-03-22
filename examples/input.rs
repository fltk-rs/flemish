use flemish::{view::*, Settings};

pub fn main() {
    flemish::application("input", State::update, State::view)
        .settings(Settings {
            size: (300, 100),
            resizable: true,
            ..Default::default()
        })
        .run();
}

#[derive(Default)]
struct State {
    text: String,
}

#[derive(Debug, Clone)]
enum Message {
    Input(String),
    Print,
}

impl State {
    fn update(&mut self, message: Message) {
        match message {
            Message::Input(s) => self.text = s,
            Message::Print => println!("Hello {}", &self.text),
        }
    }

    fn view(&self) -> View<Message> {
        Column::new(&[
            Frame::new("Enter name:").view(),
            Input::new(&self.text)
                .on_input(Message::Input)
                .on_submit(|_| Message::Print)
                .view(),
            Button::new("Submit", Message::Print).view(),
        ])
        .view()
    }
}
