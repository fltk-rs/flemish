use flemish::{view::*, Settings, Subscription};

pub fn main() {
    flemish::application("progress", State::update, State::view)
        .settings(Settings {
            size: (300, 100),
            resizable: true,
            ..Default::default()
        })
        .subscription(State::subscription)
        .run_with(State::new);
}

struct State {
    value: f64,
}

#[derive(Debug, Clone, Copy)]
enum Message {
    Tick,
}

impl State {
    fn new() -> Self {
        let value = 0.0;
        Self { value }
    }
    fn subscription(&self) -> Subscription<Message> {
        Subscription::every(std::time::Duration::from_millis(300)).map(|_| Message::Tick)
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::Tick => {
                self.value += 1.;
            }
        }
    }

    fn view(&self) -> View<Message> {
        Column::new(&[Progress::new(self.value).view()])
            .margins(10, 40, 10, 40)
            .view()
    }
}
