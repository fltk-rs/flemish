use flemish::{theme::color_themes, view::*, Settings};

pub fn main() {
    flemish::application("valuators", State::update, State::view)
        .settings(Settings {
            size: (300, 100),
            resizable: true,
            color_map: Some(color_themes::GRAY_THEME),
            ..Default::default()
        })
        .run();
}

#[derive(Default)]
struct State {
    value: f64,
}

#[derive(Debug, Clone)]
enum Message {
    Input(f64),
}

impl State {
    fn update(&mut self, message: Message) {
        match message {
            Message::Input(s) => self.value = s,
        }
    }

    fn view(&self) -> View<Message> {
        Column::new(&[
            Frame::new(&self.value.to_string()).view(),
            HorNiceSlider::new(self.value)
                .on_change(Message::Input)
                .fixed(30)
                .view(),
        ])
        .view()
    }
}
