use flemish::{enums::Shortcut, view::*, Settings};

pub fn main() {
    flemish::application("menu app", MenuApp::update, MenuApp::view)
        .settings(Settings {
            size: (300, 300),
            resizable: true,
            ignore_esc_close: true,
            ..Default::default()
        })
        .run();
}

#[derive(Default)]
struct MenuApp {
    value: i32,
}

#[derive(Debug, Clone, Copy)]
enum Message {
    Increment,
    Decrement,
}

impl MenuApp {
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
            MenuBar::new(&[
                MenuItem::new(
                    "Command/Increment",
                    Shortcut::None,
                    MenuFlag::Normal,
                    Message::Increment,
                ),
                MenuItem::new(
                    "Command/Decrement",
                    Shortcut::None,
                    MenuFlag::Normal,
                    Message::Decrement,
                ),
            ])
            .fixed(30)
            .view(),
            Frame::new(&self.value.to_string()).view(),
        ])
        .view()
    }
}
