#![forbid(unsafe_code)]

use {
    flemish::{
        app,
        button::Button,
        color_themes,
        enums::{FrameType,Shortcut,Key,Align,Font},
        frame::Frame,
        group::Flex,
        text::{WrapMode,TextBuffer,TextDisplay},
        prelude::*,
        OnEvent, Sandbox, Settings,
    },
};

pub fn main() {
    Model::new().run(Settings {
        size: (360, 640),
        resizable: false,
        ignore_esc_close: true,
        color_map: Some(color_themes::DARK_THEME),
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
}

#[derive(PartialEq, Clone)]
enum Message {
    Click(String),
}

impl Sandbox for Model {
    type Message = Message;

    fn new() -> Self {
        Self {
            prev: String::from("0"),
            operation: String::new(),
            current: String::from("0"),
            output: String::new(),
        }
    }

    fn title(&self) -> String {
        String::from("FlCalculator")
    }

    fn view(&mut self) {
        let mut page = Flex::default_fill().column();
        crate::display("Output", &self.output);
        let mut row = Flex::default();
        row.fixed(&crate::output("Operation", &self.operation), 30);
        let mut col = Flex::default().column();
        crate::output("Previous", &self.prev);
        crate::output("Current", &self.current);
        col.end();
        row.end();
        let mut buttons = Flex::default_fill().column();
        for line in BUTTONS {
            let row = Flex::default();
            for label in line {
                crate::button(label).on_event(move|_|Message::Click(label.to_string()));
            }
            row.end();
        }
        buttons.end();
        page.end();
        {
            col.set_pad(0);
            row.set_pad(0);
            row.set_margin(0);
            buttons.set_pad(PAD);
            buttons.set_margin(0);
            page.set_margin(PAD);
            page.set_pad(PAD);
            page.set_margin(PAD);
            page.fixed(&row, 60);
            page.fixed(&buttons, 425);
            page.set_frame(FrameType::FlatBox);
            app::set_font(Font::Courier);
        }
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::Click(value) =>  {
            match value.as_str() {
                "/" | "x" | "+" | "-" | "%" => {
                    if self.operation.is_empty() {
                        self.operation.push_str(&value);
                        self.prev = self.current.clone();
                    } else {
                        self.equil();
                        self.operation = String::from("=");
                    }
                    self.output.push_str(&format!("{} {}", self.prev, self.operation));
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
            }}
        };
    }
}

impl Model {
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

fn display(tooltip: &str, value: &str) {
    let mut element = TextDisplay::default();
    element.set_tooltip(tooltip);
    element.set_buffer(TextBuffer::default());
    element.buffer().unwrap().set_text(value);
    element.set_text_size(HEIGHT - 5);
    element.set_scrollbar_size(3);
    element.set_frame(FrameType::FlatBox);
    element.wrap_mode(WrapMode::AtBounds, 0);
    element.deactivate();
    element.scroll(
        element.buffer().unwrap().text().split_whitespace().count() as i32,
        0,
    );
}

fn output(tooltip: &str, value: &str) -> Frame {
    let mut element = Frame::default()
        .with_align(Align::Right | Align::Inside);
    element.set_tooltip(tooltip);
    element.set_label_size(HEIGHT);
    element.set_label(value);
    element.set_frame(FrameType::FlatBox);
    element
}

fn button(label: &'static str) -> Button {
    let mut element = Button::default().with_label(label);
    element.set_label_size(HEIGHT);
    element.set_frame(FrameType::RoundUpBox);
    match label {
        "@<-" => element.set_shortcut(Shortcut::None | Key::BackSpace),
        "CE" => element.set_shortcut(Shortcut::None | Key::Delete),
        "=" => element.set_shortcut(Shortcut::None | Key::Enter),
        "x" => element.set_shortcut(Shortcut::None | '*'),
        _ => element.set_shortcut(Shortcut::None | label.chars().next().unwrap()),
    }
    element
}

const BUTTONS: [[&str; 4]; 5] = [
    ["CE", "C", "%", "/"],
    ["7", "8", "9", "x"],
    ["4", "5", "6", "-"],
    ["1", "2", "3", "+"],
    ["0", ".", "@<-", "="],
];
const PAD: i32 = 10;
const HEIGHT: i32 = PAD * 3;
