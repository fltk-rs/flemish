use flemish::{
    color_themes, frame::Frame, group::Flex, enums::Shortcut, menu::{MenuBar, MenuFlag}, prelude::*, OnMenuEvent, Sandbox,
    Settings,
};

pub fn main() {
    MenuApp::new().run(Settings {
        size: (300, 300),
        resizable: true,
        ignore_esc_close: true,
        color_map: Some(color_themes::BLACK_THEME),
        ..Default::default()
    })
}

#[derive(Default)]
struct MenuApp {
    value: i32,
}

#[derive(Debug, Clone, Copy)]
enum Message {
    IncrementPressed,
    DecrementPressed,
}

impl Sandbox for MenuApp {
    type Message = Message;

    fn new() -> Self {
        Self::default()
    }

    fn title(&self) -> String {
        String::from("MenuApp - fltk-rs")
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::IncrementPressed => {
                self.value += 1;
            }
            Message::DecrementPressed => {
                self.value -= 1;
            }
        }
    }

    fn view(&mut self) {
        let mut col = Flex::default_fill().column();
        let m = MenuBar::default()
            .on_item_event(
                "Command/Increment",
                Shortcut::None,
                MenuFlag::Normal,
                Message::IncrementPressed,
            )
            .on_item_event(
                "Command/Decrement",
                Shortcut::None,
                MenuFlag::Normal,
                Message::DecrementPressed,
            );
        col.fixed(&m, 40);
        Frame::default().with_label(&self.value.to_string());
        col.end();
    }
}
