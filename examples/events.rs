use flemish::{enums::Event, view::*, Settings, Subscription};

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
    Event(Event),
}

impl Timer {
    fn subscription(&self) -> Subscription<Message> {
        Subscription::events().map(Message::Event)
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::Event(i) => {
                println!("{:?}", i);
                self.value += 1;
            }
        }
    }

    fn view(&self) -> View<Message> {
        Frame::new(&self.value.to_string()).view()
    }
}
