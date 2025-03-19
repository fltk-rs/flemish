use flemish::{
    enums::{Color, FrameType},
    view::*,
    Settings,
};

pub fn main() {
    flemish::application("colors", Colors::update, Colors::view)
        .settings(Settings {
            size: (300, 100),
            resizable: true,
            ..Default::default()
        })
        .run();
}

#[derive(Default)]
struct Colors {
    value: i8,
}

#[derive(Debug, Clone, Copy)]
enum Message {
    Increment,
    Decrement,
}

impl Colors {
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
            Frame::new(&self.value.to_string())
                .boxtype(FrameType::FlatBox)
                .color(Color::by_index(self.value as _))
                .label_color(Color::contrast(
                    Color::Gray0,
                    Color::by_index(self.value as _),
                ))
                .view(),
            Button::new("-", Message::Decrement).view(),
        ])
        .view()
    }
}
