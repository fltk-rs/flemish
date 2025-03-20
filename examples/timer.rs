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
    duration: u64,
}

#[derive(Debug, Clone, Copy)]
enum Message {
    Tick(std::time::Instant),
    Reduce,
}

impl Timer {
    fn new() -> Self {
        let value = 0;
        let time = Instant::now();
        let duration = 1000;
        Self {
            value,
            time,
            duration,
        }
    }
    fn subscription(&self) -> Subscription<Message> {
        Subscription::every(std::time::Duration::from_millis(self.duration)).map(Message::Tick)
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::Tick(i) => {
                println!("{:?}", i.duration_since(self.time));
                self.value += 1;
            }
            Message::Reduce => {
                self.duration -= 100;
            }
        }
    }

    fn view(&self) -> View<Message> {
        Column::new(&[
            Frame::new(&self.value.to_string()).view(),
            Button::new("Reduce duration", Message::Reduce).view(),
        ])
        .view()
    }
}
