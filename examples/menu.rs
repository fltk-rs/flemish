use flemish::{
    app, color_themes,
    enums::Shortcut,
    frame::Frame,
    group::Flex,
    menu::{MenuBar, MenuFlag},
    prelude::*,
    OnMenuEvent, Sandbox, Settings,
};

pub fn main() {
    Model::new().run(Settings {
        size: (640, 360),
        resizable: true,
        ignore_esc_close: true,
        color_map: Some(color_themes::DARK_THEME),
        scheme: Some(app::Scheme::Base),
        ..Default::default()
    })
}

#[derive(Default)]
struct Model {
    value: i32,
}

#[derive(Debug, Clone, Copy)]
enum Message {
    IncrementPressed,
    DecrementPressed,
}

impl Sandbox for Model {
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
                |_| Message::IncrementPressed,
            )
            .on_item_event(
                "Command/Decrement",
                Shortcut::None,
                MenuFlag::Normal,
                |_| Message::DecrementPressed,
            );
        col.fixed(&m, 40);
        Frame::default().with_label(&self.value.to_string());
        col.end();
    }
}
