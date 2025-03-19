use flemish::{theme::color_themes, view::*, Settings, Subscription};
use std::time::Instant;

pub fn main() {
    flemish::application("timer", Timer::update, Timer::view)
        .settings(Settings {
            size: (300, 100),
            resizable: true,
            color_map: Some(color_themes::BLACK_THEME),
            ..Default::default()
        })
        .subscription(Timer::subscription)
        .run_with(Timer::new);
}

struct Timer {
    value: i32,
    time: Instant,
}

#[derive(Debug, Clone, Copy)]
enum Message {
    Tick(std::time::Instant),
}

impl Timer {
    fn new() -> Self {
        let value = 0;
        let time = Instant::now();
        Self { value, time }
    }
    fn subscription(&self) -> Subscription<Message> {
        Subscription::every(std::time::Duration::from_secs(1)).map(Message::Tick)
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::Tick(i) => {
                println!("{:?}", i.duration_since(self.time));
                self.value += 1;
            }
        }
    }

    fn view(&self) -> View<Message> {
        Frame::new(&self.value.to_string()).view()
    }
}
