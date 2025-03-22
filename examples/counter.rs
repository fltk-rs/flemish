use flemish::{view::*, Settings};

pub fn main() {
    flemish::application("counter", Counter::update, Counter::view)
        .settings(Settings {
            size: (300, 100),
            resizable: true,
            ..Default::default()
        })
        .run();
}

#[derive(Default)]
struct Counter {
    value: i32,
}

#[derive(Debug, Clone, Copy)]
enum Message {
    Increment,
    Decrement,
}

impl Counter {
    fn update(&mut self, message: Message) {
        match message {
            Message::Increment => {
                self.value += 1;
            }
            Message::Decrement => {
                self.value -= 1;
            }
        }
    }

    fn view(&self) -> View<Message> {
        Column::new(&[
            Button::new("+", Message::Increment).view(),
            Frame::new(&self.value.to_string()).view(),
            Button::new("-", Message::Decrement).view(),
        ])
        .view()
    }
}
