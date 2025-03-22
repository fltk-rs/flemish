use flemish::{view::*, Settings, Subscription, Task};

pub fn main() {
    flemish::application("timer", Timer::update, Timer::view)
        .settings(Settings {
            size: (300, 100),
            resizable: true,
            ..Default::default()
        })
        .subscription(Timer::subscription)
        .run();
}

#[derive(Default)]
struct Timer {
    value: i32,
}

#[derive(Debug, Clone, Copy)]
enum Message {
    Tick,
    Print(i32),
}

impl Timer {
    fn subscription(&self) -> Subscription<Message> {
        Subscription::every(std::time::Duration::from_secs(1)).map(|_| Message::Tick)
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Tick => {
                self.value += 1;
                return Task::perform_async(|| async { 2 + 2 }).map(Message::Print);
            }
            Message::Print(v) => {
                println!("{}", v);
            }
        }
        Task::none()
    }

    fn view(&self) -> View<Message> {
        Frame::new(&self.value.to_string()).view()
    }
}
