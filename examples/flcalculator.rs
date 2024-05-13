#![forbid(unsafe_code)]

use {
    flemish::{
        app,
        button::Button,
        color_themes,
        enums::{Align, Color, Event, Font, FrameType, Key, Shortcut},
        frame::Frame,
        group::Flex,
        menu::{MenuButton, MenuButtonType, MenuFlag},
        prelude::*,
        text::{TextBuffer, TextDisplay, WrapMode},
        OnEvent, OnMenuEvent, Sandbox, Settings,
    },
    std::{env, fs, path::Path},
};

pub fn main() {
    Model::new().run(Settings {
        size: (360, 640),
        resizable: false,
        ignore_esc_close: true,
        color_map: Some(color_themes::TAN_THEME),
        scheme: Some(app::Scheme::Base),
        ..Default::default()
    })
}

#[derive(Clone)]
struct Model {
    prev: String,
    operation: String,
    current: String,
    output: String,
    theme: bool,
}

#[derive(PartialEq, Clone)]
enum Message {
    Click(String),
    Theme,
    Quit,
}

impl Sandbox for Model {
    type Message = Message;

    fn new() -> Self {
        let file = env::var("HOME").unwrap() + PATH + NAME;
        let theme: bool = match Path::new(&file).exists() {
            true => fs::read(&file).unwrap()[0],
            false => 0,
        } != 0;
        Self {
            prev: String::from("0"),
            operation: String::new(),
            current: String::from("0"),
            output: String::new(),
            theme,
        }
    }

    fn title(&self) -> String {
        String::from(NAME)
    }

    fn view(&mut self) {
        let menu = crate::menu(self.theme as usize);
        let mut page = Flex::default_fill().column();
        crate::display("Output", &self.output, self.theme as usize);
        let mut row = Flex::default();
        row.fixed(
            &crate::output("Operation", &self.operation, self.theme as usize),
            30,
        );
        let mut col = Flex::default().column();
        crate::output("Previous", &self.prev, self.theme as usize);
        crate::output("Current", &self.current, self.theme as usize);
        col.end();
        row.end();
        let mut buttons = Flex::default_fill().column();
        for line in BUTTONS {
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
            buttons.handle(move |_, event| match event {
                Event::Push => match app::event_mouse_button() {
                    app::MouseButton::Right => {
                        menu.popup();
                        true
                    }
                    _ => false,
                },
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
            Message::Quit => self.quit(),
            Message::Theme => {
                self.theme = !self.theme;
            }
            Message::Click(value) => match value.as_str() {
                "/" | "x" | "+" | "-" | "%" => {
                    if self.operation.is_empty() {
                        self.operation.push_str(&value);
                        self.prev = self.current.clone();
                    } else {
                        self.equil();
                        self.operation = String::from("=");
                    }
                    self.output
                        .push_str(&format!("{} {}", self.prev, self.operation));
                    self.current = String::from("0");
                }
                "=" => self.equil(),
                "CE" => {
                    self.output.clear();
                    self.operation.clear();
                    self.current = String::from("0");
                    self.prev = String::from("0");
                }
                "@<-" => {
                    let label = self.current.clone();
                    self.current = if label.len() > 1 {
                        String::from(&label[..label.len() - 1])
                    } else {
                        String::from("0")
                    };
                }
                "C" => self.current = String::from("0"),
                "." => {
                    if !self.current.contains('.') {
                        self.current.push('.');
                    }
                }
                _ => {
                    if self.current == "0" {
                        self.current.clear();
                    }
                    self.current = self.current.clone() + &value;
                }
            },
        };
    }
}

impl Model {
    fn quit(&self) {
        let file = env::var("HOME").unwrap() + PATH + NAME;
        fs::write(file, [self.theme as u8]).unwrap();
        app::quit();
    }
    fn equil(&mut self) {
        if !self.operation.is_empty() {
            let left: f64 = self.prev.parse().unwrap();
            let right: f64 = self.current.parse().unwrap();
            let temp = match self.operation.as_str() {
                "/" => left / right,
                "x" => left * right,
                "+" => left + right,
                "-" => left - right,
                _ => left / 100.0 * right,
            };
            self.output.push_str(&format!(
                " {right}\n{} = {temp}\n",
                (0..=left.to_string().len())
                    .map(|_| ' ')
                    .collect::<String>(),
            ));
            self.prev = temp.to_string();
        } else {
            self.prev = self.current.clone();
        }
        self.operation.clear();
        self.current = String::from("0");
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
    element.set_color(COLORS[theme as usize][0]);
    element.set_text_color(COLORS[theme as usize][1]);
    element.scroll(
        element.buffer().unwrap().text().split_whitespace().count() as i32,
        0,
    );
}

fn output(tooltip: &str, value: &str, theme: usize) -> Frame {
    let mut element = Frame::default().with_align(Align::Right | Align::Inside);
    element.set_tooltip(tooltip);
    element.set_label_size(HEIGHT);
    element.set_label(value);
    element.set_frame(FrameType::FlatBox);
    element.set_color(COLORS[theme as usize][0]);
    element.set_label_color(COLORS[theme as usize][1]);
    element
}

fn button(label: &'static str, theme: usize) -> Button {
    let mut element = Button::default().with_label(label);
    element.set_label_size(HEIGHT);
    element.set_frame(FrameType::OFlatFrame);
    match label {
        "@<-" => element.set_shortcut(Shortcut::None | Key::BackSpace),
        "CE" => element.set_shortcut(Shortcut::None | Key::Delete),
        "=" => element.set_shortcut(Shortcut::None | Key::Enter),
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
        "=" => {
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
        .clone()
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
        element.at(1).unwrap().set();
    };
    element
}

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
const BUTTONS: [[&str; 4]; 5] = [
    ["CE", "C", "%", "/"],
    ["7", "8", "9", "x"],
    ["4", "5", "6", "-"],
    ["1", "2", "3", "+"],
    ["0", ".", "@<-", "="],
];
const PAD: i32 = 10;
const HEIGHT: i32 = PAD * 3;
const NAME: &str = "FlCalculator";
const PATH: &str = "/.config";
