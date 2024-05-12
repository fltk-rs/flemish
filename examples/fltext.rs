#![forbid(unsafe_code)]

use {
    flemish::{
        app,
        button::Button,
        color_themes,
        enums::{Align, Color, Font, FrameType, Shortcut},
        frame::Frame,
        group::{Flex, Tabs, TabsOverflow},
        input::Input,
        menu::{MenuButton, MenuFlag},
        prelude::*,
        text::{StyleTableEntry, TextBuffer, TextEditor, WrapMode},
        tree::Tree,
        OnEvent, OnMenuEvent, Sandbox, Settings,
    },
    std::{process::Command, thread},
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

#[derive(Clone, Copy)]
struct Style {
    font: u8,
    size: u8,
}

#[derive(Clone)]
struct Object {
    change: bool,
    path: String,
    file: String,
    text: String,
}

#[derive(Clone)]
struct Model {
    style: Style,
    objects: Vec<Object>,
}

#[derive(Clone)]
enum Message {
    Open,
    Save,
    New,
    Quit,
}

impl Sandbox for Model {
    type Message = Message;

    fn new() -> Self {
        Self {
            style: Style { font: 0, size: 14 },
            objects: Vec::from([Object {
                change: false,
                path: String::from("."),
                file: String::from("untitled_0.txt"),
                text: String::new(),
            }]),
        }
    }

    fn title(&self) -> String {
        String::from(NAME)
    }

    fn view(&mut self) {
        let mut page = Flex::default_fill(); //PAGE
        let mut left = Flex::default().column(); //LEFT
        left.fixed(&crate::menu(), HEIGHT);
        let mut files = Tree::default();
        left.end();
        let mut right = Flex::default_fill();
        let mut tabs = Tabs::default(); //RIGHT
        for obj in &self.objects {
            let tab = Flex::default()
                .with_label(&format!("    {}    ", obj.file))
                .column();
            crate::text(&obj.file, &obj.text, self.style);
            tab.end();
            files.add(&format!("{}/{}", obj.path, obj.file));
        }
        tabs.end();
        right.end();
        page.end();
        {
            files.set_show_root(false);
            left.set_pad(PAD);
            left.set_margin(PAD);
            right.set_frame(FrameType::DownBox);
            tabs.auto_layout();
            tabs.handle_overflow(TabsOverflow::Pulldown);
            tabs.set_tab_align(Align::Right);
            page.set_pad(0);
            page.set_margin(0);
            page.set_frame(FrameType::FlatBox);
            page.fixed(&left, WIDTH);
        }
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::New => self.objects.push(Object {
                change: false,
                path: String::from("."),
                file: String::from(format!("untitled_{}.txt", self.objects.len())),
                text: String::new(),
            }),
            Message::Quit => app::quit(),
            Message::Save => {}
            Message::Open => {}
        }
    }
}

fn text(tooltip: &str, value: &str, style: Style) -> TextEditor {
    let mut element = TextEditor::default();
    element.set_tooltip(tooltip);
    element.set_linenumber_width(HEIGHT);
    element.set_buffer(TextBuffer::default());
    element.wrap_mode(WrapMode::AtBounds, 0);
    element.buffer().unwrap().set_text(value);
    element.set_color(Color::from_hex(0x002b36));
    element.set_text_color(Color::from_hex(0x93a1a1));
    element.set_text_font(Font::by_index(style.font as usize));
    element.set_text_size(style.size as i32);
    element
}

fn menu() -> MenuButton {
    let element = MenuButton::default().with_label("@#menu");
    element
        .clone()
        .on_item_event(
            "@#+  &New",
            Shortcut::Ctrl | 'n',
            MenuFlag::Normal,
            move |_| Message::New,
        )
        .on_item_event(
            "@#fileopen  &Open",
            Shortcut::Ctrl | 'o',
            MenuFlag::Normal,
            move |_| Message::New,
        )
        .on_item_event(
            "@#filesave  &Save",
            Shortcut::Ctrl | 'o',
            MenuFlag::Normal,
            move |_| Message::New,
        )
        .on_item_event(
            "@#1+  &Quit",
            Shortcut::Ctrl | 'q',
            MenuFlag::Normal,
            move |_| Message::Quit,
        );
    element
}

const PAD: i32 = 10;
const WIDTH: i32 = 150;
const HEIGHT: i32 = PAD * 3;
const NAME: &str = "FlText";
