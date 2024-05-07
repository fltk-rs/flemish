#![forbid(unsafe_code)]

use {
    flemish::{
        app, button::Button, color_themes, frame::Frame, group::Flex, prelude::*,
        OnEvent, Sandbox, Settings, OnMenuEvent,
        enums::Shortcut,
        menu::{MenuFlag,MenuButton},
    },
    serde::{Deserialize, Serialize},
    std::{env, fs},
};

pub fn main() {
    let mut app = Counter::new();
    app.run(Settings {
        size: (app.width, app.height),
        pos: (app.vertical,app.horizontal),
        resizable: true,
        color_map: Some(color_themes::DARK_THEME),
        scheme: Some(app::Scheme::Plastic),
        ..Default::default()
    })
}

const NAME: &str = "Counter";
const PAD: i32 = 10;
const HEIGHT: i32 = PAD * 3;
const WIDTH: i32 = HEIGHT * 3;
const WINDOW_WIDTH: i32 = 640;
const WINDOW_HEIGHT: i32 = 360;

#[derive(Deserialize, Serialize)]
struct Counter {
    file: String,
    width: i32,
    height: i32,
    vertical: i32,
    horizontal: i32,
    value: i32,
}

#[derive(Debug, Clone, Copy)]
enum Message {
    Inc,
    Dec,
    Quit,
}

impl Sandbox for Counter {
    type Message = Message;

    fn title(&self) -> String {
        String::from(NAME)
    }

    fn new() -> Self {
        let file = env::var("HOME").unwrap() + "/.config/" + NAME;
        let default = Self {
            file: file.clone(),
            width: WINDOW_WIDTH,
            height: WINDOW_HEIGHT,
            vertical: (app::screen_size().0 / 2.0) as i32 - 320,
            horizontal: (app::screen_size().1 / 2.0) as i32 - 180,
            value: 0,
        };
        let mut model: Self = if let Ok(value) = fs::read(&file) {
            if let Ok(value) = rmp_serde::from_slice(&value) {
                value
            } else {
                default
            }
        } else {
            default
        };
        model.file = file.clone();
        model
    }

    fn view(&mut self) {
        let mut page = Flex::default_fill().column();
        let mut header = Flex::default_fill();
        crate::menu("Menu", &mut header);
        header.end();
        Frame::default();
        crate::frame(&self.value.to_string(), &mut page);
        let mut row = Flex::default();
        Frame::default();
        crate::button("@#<", &mut row).on_event(|_| Message::Dec);
        crate::button("@#>", &mut row).on_event(|_| Message::Inc);
        Frame::default();
        row.end();
        Frame::default();
        page.end();
        {
            row.set_pad(0);
            page.set_pad(PAD);
            page.set_margin(PAD);
            page.fixed(&header, HEIGHT);
            page.fixed(&row, WIDTH);
            let mut window = page.window().unwrap();
            window.set_label(&format!("{} - {NAME}", self.value));
            window.size_range(WINDOW_WIDTH, WINDOW_HEIGHT, 0, 0);
        }
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::Inc => {
                self.value += 1;
            }
            Message::Dec => {
                self.value -= 1;
            }
            Message::Quit => {
                let window = app::first_window().unwrap();
                self.width = window.width();
                self.height = window.height();
                self.vertical = window.y();
                self.horizontal = window.x();
                fs::write(&self.file, rmp_serde::to_vec(&self).unwrap()).unwrap();
                app::quit();
            }
        }
    }
}

fn menu(tooltip: &str, flex: &mut Flex) -> MenuButton {
    let mut element = MenuButton::default().with_label("@#menu")
        .on_item_event(
            "@#1+  &Quit",
            Shortcut::Ctrl | 'q',
            MenuFlag::Normal,
            |_| Message::Quit,
        );
    element.set_tooltip(tooltip);
    flex.fixed(&element, 50);
    element
}

fn frame(tooltip: &str, flex: &mut Flex) -> Frame {
    let mut element = Frame::default().with_label(tooltip);
    element.set_label_size(WIDTH);
    flex.fixed(&element, WIDTH);
    element
}

fn button(tooltip: &str, flex: &mut Flex) -> Button {
    let element = Button::default().with_label(tooltip);
    flex.fixed(&element, WIDTH);
    element
}
