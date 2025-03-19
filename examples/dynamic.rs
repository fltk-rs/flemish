use flemish::{theme::color_themes, view::*, Settings};

pub fn main() {
    flemish::application("dynamic", Counter::update, Counter::view)
        .settings(Settings {
            size: (300, 100),
            resizable: true,
            color_map: Some(color_themes::BLACK_THEME),
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
        let mut fs = vec![];
        for i in 0..self.value {
            fs.push(Frame::new(&i.to_string()).view());
        }
        Column::new(&[
            Button::new("+", Message::Increment).view(),
            Row::new(&fs).view(),
            Button::new("-", Message::Decrement).view(),
        ])
        .view()
    }
}
