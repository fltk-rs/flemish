#![forbid(unsafe_code)]
mod model;

use {
    flemish::{
        app,
        button::Button,
        color_themes,
        enums::{Align, Color, Cursor, Event, Font, FrameType, Key, Shortcut},
        frame::Frame,
        group::Flex,
        menu::{MenuButton, MenuButtonType, MenuFlag},
        prelude::*,
        text::{TextBuffer, TextDisplay, WrapMode},
        OnEvent, OnMenuEvent, Sandbox, Settings,
    },
    model::Model,
};

const PAD: i32 = 10;
const HEIGHT: i32 = PAD * 3;
const EQUAL: &str = "=";
const COLORS: [[Color; 6]; 2] = [
    [
        Color::from_hex(0xfdf6e3),
        Color::from_hex(0x586e75),
        Color::from_hex(0xb58900),
        Color::from_hex(0xeee8d5),
        Color::from_hex(0xcb4b16),
        Color::from_hex(0xdc322f),
    ],
    [
        Color::from_hex(0x002b36),
        Color::from_hex(0x93a1a1),
        Color::from_hex(0x268bd2),
        Color::from_hex(0x073642),
        Color::from_hex(0x6c71c4),
        Color::from_hex(0xd33682),
    ],
];
const NAME: &str = "FlCalculator";

fn main() {
    app::GlobalState::<String>::new(std::env::var("HOME").unwrap() + "/.config/" + NAME);
    Model::new().run(Settings {
        size: (360, 640),
        resizable: false,
        ignore_esc_close: true,
        color_map: Some(color_themes::TAN_THEME),
        scheme: Some(app::Scheme::Base),
        ..Default::default()
    })
}

#[derive(PartialEq, Clone)]
pub enum Message {
    Click(String),
    Theme,
    Quit,
}

impl Sandbox for Model {
    type Message = Message;

    fn title(&self) -> String {
        String::from(NAME)
    }

    fn new() -> Self {
        let file = app::GlobalState::<String>::get().with(move |file| file.clone());
        Model::default(&file)
    }

    fn view(&mut self) {
        let menu = crate::menu(self.theme as usize);
        let mut page = Flex::default_fill().column();
        crate::display("Output", &self.output, self.theme as usize);
        let mut row = Flex::default();
        row.fixed(
            &crate::output("Operation", self.theme as usize).with_label(&self.operation),
            30,
        );
        let mut col = Flex::default().column();
        crate::output("Previous", self.theme as usize).with_label(&self.prev.to_string());
        crate::output("Current", self.theme as usize).with_label(&self.current);
        col.end();
        row.end();
        let mut buttons = Flex::default_fill().column();
        for line in [
            ["CE", "C", "%", "/"],
            ["7", "8", "9", "x"],
            ["4", "5", "6", "-"],
            ["1", "2", "3", "+"],
            ["0", ".", "@<-", crate::EQUAL],
        ] {
            let mut row = Flex::default();
            for label in line {
                crate::button(label, self.theme as usize)
                    .on_event(move |_| Message::Click(label.to_string()));
            }
            row.end();
            row.set_pad(PAD);
            row.set_margin(0);
        }
        buttons.end();
        page.end();
        {
            col.set_pad(0);
            row.set_pad(0);
            row.set_margin(0);
            buttons.set_pad(PAD);
            buttons.set_margin(0);
            buttons.handle(move |flex, event| match event {
                Event::Push => match app::event_mouse_button() {
                    app::MouseButton::Right => {
                        menu.popup();
                        true
                    }
                    _ => false,
                },
                Event::Enter => {
                    flex.window().unwrap().set_cursor(Cursor::Hand);
                    true
                }
                Event::Leave => {
                    flex.window().unwrap().set_cursor(Cursor::Arrow);
                    true
                }
                _ => false,
            });
            page.set_margin(PAD);
            page.set_pad(PAD);
            page.set_margin(PAD);
            page.fixed(&row, 60);
            page.fixed(&buttons, 425);
            page.set_frame(FrameType::FlatBox);
            page.set_color(COLORS[self.theme as usize][0]);
            app::set_font(Font::Courier);
        }
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::Quit => {
                let file = app::GlobalState::<String>::get().with(move |file| file.clone());
                std::fs::write(file, rmp_serde::to_vec(&self).unwrap()).unwrap();
                app::quit();
            }
            Message::Theme => self.theme = !self.theme,
            Message::Click(value) => self.click(&value),
        };
    }
}

fn display(tooltip: &str, value: &str, theme: usize) {
    let mut element = TextDisplay::default();
    element.set_tooltip(tooltip);
    element.set_buffer(TextBuffer::default());
    element.buffer().unwrap().set_text(value);
    element.set_text_size(HEIGHT - 5);
    element.set_scrollbar_size(3);
    element.set_frame(FrameType::FlatBox);
    element.wrap_mode(WrapMode::AtBounds, 0);
    element.set_color(COLORS[theme][0]);
    element.set_text_color(COLORS[theme][1]);
    element.scroll(
        element.buffer().unwrap().text().split_whitespace().count() as i32,
        0,
    );
}

fn output(tooltip: &str, theme: usize) -> Frame {
    let mut element = Frame::default().with_align(Align::Right | Align::Inside);
    element.set_tooltip(tooltip);
    element.set_label_size(HEIGHT);
    element.set_frame(FrameType::FlatBox);
    element.set_color(COLORS[theme][0]);
    element.set_label_color(COLORS[theme][1]);
    element
}

fn button(label: &'static str, theme: usize) -> Button {
    let mut element = Button::default().with_label(label);
    element.set_label_size(HEIGHT);
    element.set_frame(FrameType::OFlatFrame);
    match label {
        "@<-" => element.set_shortcut(Shortcut::None | Key::BackSpace),
        "CE" => element.set_shortcut(Shortcut::None | Key::Delete),
        crate::EQUAL => element.set_shortcut(Shortcut::None | Key::Enter),
        "x" => element.set_shortcut(Shortcut::None | '*'),
        _ => element.set_shortcut(Shortcut::None | label.chars().next().unwrap()),
    }
    match label {
        "C" | "x" | "/" | "+" | "-" | "%" => {
            element.set_color(COLORS[theme][2]);
            element.set_label_color(COLORS[theme][0]);
        }
        "CE" => {
            element.set_color(COLORS[theme][4]);
            element.set_label_color(COLORS[theme][0]);
        }
        crate::EQUAL => {
            element.set_color(COLORS[theme][5]);
            element.set_label_color(COLORS[theme][0]);
        }
        _ => {
            element.set_color(COLORS[theme][3]);
            element.set_label_color(COLORS[theme][1]);
        }
    };
    element
}

pub fn menu(theme: usize) -> MenuButton {
    let mut element = MenuButton::default()
        .with_type(MenuButtonType::Popup3)
        .with_label("@menu");
    element.set_tooltip("Menu");
    element.set_frame(FrameType::FlatBox);
    element.set_color(COLORS[theme][1]);
    element.set_text_color(COLORS[theme][0]);
    element
        .on_item_event(
            "&Night mode\t",
            Shortcut::Ctrl | 'n',
            MenuFlag::Toggle,
            move |_| Message::Theme,
        )
        .on_item_event(
            "@#1+  &Quit",
            Shortcut::Ctrl | 'q',
            MenuFlag::Normal,
            move |_| Message::Quit,
        );
    if theme != 0 {
        element.at(0).unwrap().set();
    };
    element
}
