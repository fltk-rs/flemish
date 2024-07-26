#![forbid(unsafe_code)]

use flemish::{
    app,
    button::Button,
    color_themes,
    dialog::{choice2_default, FileChooser, FileChooserType},
    enums::{FrameType, Shortcut},
    frame::Frame,
    group::Flex,
    image::SharedImage,
    menu::{MenuButton, MenuFlag},
    prelude::*,
    OnEvent, OnMenuEvent, Sandbox, Settings,
};
use std::fs;

pub fn main() {
    Model::new().run(Settings {
        size: (640, 360),
        color_map: Some(color_themes::DARK_THEME),
        ..Default::default()
    })
}

const PAD: i32 = 10;
const HEIGHT: i32 = PAD * 3;

#[derive(Debug, Clone)]
struct Image {
    file: String,
    image: SharedImage,
}

#[derive(Debug, Clone)]
struct Model {
    list: Vec<Image>,
    curr: usize,
}

#[derive(Clone, Copy)]
enum Message {
    Open,
    Next,
    Prev,
    Remove,
    Quit,
}

impl Sandbox for Model {
    type Message = Message;

    fn new() -> Self {
        Self {
            list: Vec::new(),
            curr: 0,
        }
    }

    fn title(&self) -> String {
        String::from("FlPictures")
    }

    fn view(&mut self) {
        let mut page = Flex::default_fill().column();
        {
            let mut header = Flex::default();
            crate::menu(&mut header);
            crate::button("Open", "@#fileopen", &mut header).on_event(|_| Message::Open);
            crate::button("Prev", "@#|<", &mut header).on_event(|_| Message::Prev);
            Frame::default();
            crate::button("Next", "@#>|", &mut header).on_event(|_| Message::Next);
            crate::button("Remove", "@#1+", &mut header).on_event(|_| Message::Remove);
            header.end();
            header.set_pad(0);
            header.set_margin(0);
            page.fixed(&header, HEIGHT);
            crate::frame("Image", self.clone());
        }
        page.end();
        page.set_pad(PAD);
        page.set_margin(PAD);
        page.set_frame(FrameType::FlatBox);
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::Open => self.open(),
            Message::Prev => self.prev(),
            Message::Next => self.next(),
            Message::Remove => self.remove(),
            Message::Quit => app::quit(),
        }
    }
}

impl Model {
    fn open(&mut self) {
        let mut dialog = FileChooser::new(
            std::env::var("HOME").unwrap(),
            "*.{png,svg}",
            FileChooserType::Multi,
            "Choose File...",
        );
        dialog.show();
        while dialog.shown() {
            app::wait();
        }
        if dialog.count() > 0 {
            for item in 1..=dialog.count() {
                if let Some(file) = dialog.value(item) {
                    if let Ok(image) = SharedImage::load(file.clone()) {
                        self.list.push(Image { file, image });
                    };
                };
            }
            self.curr = 0;
        };
    }

    fn prev(&mut self) {
        if !self.list.is_empty() {
            self.curr = match self.curr > 0 {
                true => self.curr.saturating_sub(1),
                false => self.list.len() - 1,
            };
        }
    }

    fn next(&mut self) {
        if !self.list.is_empty() {
            self.curr = match self.curr < self.list.len() - 1 {
                true => self.curr.saturating_add(1),
                false => 0,
            };
        }
    }

    fn remove(&mut self) {
        if !self.list.is_empty() {
            match choice2_default("Remove ...?", "Remove", "Cancel", "Permanent") {
                Some(0) => {
                    self.list.remove(self.curr);
                }
                Some(2) => {
                    if fs::remove_file(self.list[self.curr].file.clone()).is_ok() {
                        self.list.remove(self.curr);
                    }
                }
                _ => {}
            };
            self.next();
        }
    }
}

fn button(tooltip: &str, label: &str, flex: &mut Flex) -> Button {
    let mut element = Button::default().with_label(label);
    element.set_tooltip(tooltip);
    flex.fixed(&element, crate::HEIGHT);
    element
}

fn frame(tooltip: &str, value: Model) -> Frame {
    let mut element = Frame::default();
    element.set_tooltip(tooltip);
    element.set_image(match value.list.is_empty() {
        true => None::<SharedImage>,
        false => Some(value.list[value.curr].image.clone()),
    });
    element.set_frame(FrameType::DownBox);
    element
}

fn menu(flex: &mut Flex) {
    let mut element = MenuButton::default().with_label("@#menu");
    element.set_tooltip("Main menu");
    flex.fixed(&element, 50);
    element
        .on_item_event(
            "&File/@#fileopen  &Open",
            Shortcut::Ctrl | 'o',
            MenuFlag::Normal,
            |_| Message::Open,
        )
        .on_item_event(
            "&File/@#1+  &Remove",
            Shortcut::Ctrl | 'd',
            MenuFlag::Normal,
            |_| Message::Remove,
        )
        .on_item_event(
            "&Image/@#>|  &Next",
            Shortcut::Ctrl | 'n',
            MenuFlag::Normal,
            |_| Message::Next,
        )
        .on_item_event(
            "&Image/@#|<  &Prev",
            Shortcut::Ctrl | 'p',
            MenuFlag::Normal,
            |_| Message::Prev,
        )
        .on_item_event(
            "@#1+  &Quit",
            Shortcut::Ctrl | 'q',
            MenuFlag::Normal,
            |_| Message::Quit,
        );
}
