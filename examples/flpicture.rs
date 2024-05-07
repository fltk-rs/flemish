use flemish::{
    app,
    button::Button,
    color_themes,
    dialog::{choice2_default, FileChooser, FileChooserType},
    enums::Shortcut,
    frame::Frame,
    group::Flex,
    image::SharedImage,
    valuator::{Slider,SliderType},
    menu::{MenuButton, MenuFlag},
    prelude::*,
    OnEvent, OnMenuEvent, Sandbox, Settings,
};
use std::fs;

pub fn main() {
    Model::new().run(Settings {
        size: (640, 360),
        resizable: true,
        ignore_esc_close: true,
        color_map: Some(color_themes::DARK_THEME),
        scheme: Some(app::Scheme::Plastic),
        ..Default::default()
    })
}

const HEIGHT: i32 = 30;
const PAD: i32 = 10;

struct Model {
    images: Vec<String>,
    size: f64,
    current: usize,
}

#[derive(Clone, Copy)]
enum Message {
    Open,
    Next,
    Prev,
    Remove,
    Size(f64),
    Quit,
}

impl Sandbox for Model {
    type Message = Message;

    fn new() -> Self {
        Self {
            images: Vec::new(),
            size: 1f64,
            current: 0,
        }
    }

    fn title(&self) -> String {
        String::from("FlPictures")
    }

    fn view(&mut self) {
        let mut page = Flex::default_fill().column();
        let mut header = Flex::default();
        crate::menu(&mut header);
        crate::button("Open", "@#fileopen", &mut header).on_event(|_|Message::Open);
        crate::button("Prev", "@#|<", &mut header).on_event(|_|Message::Prev);
        let size = crate::slider("Size", self.size).with_type(SliderType::Horizontal);
        size.clone().on_event(move |_|Message::Size(size.value()));
        crate::button("Next", "@#>|", &mut header).on_event(|_|Message::Next);
        crate::button("Remove", "@#1+", &mut header).on_event(|_|Message::Remove);
        header.end();
        let mut frame = crate::frame("Image");
        if self.images.is_empty() {
            frame.set_image(None::<SharedImage>);
        } else if let Ok(mut image) = SharedImage::load(self.images[self.current].clone()) {
            image.scale(
                (frame.width() as f64 * self.size) as i32,
                (frame.height() as f64 * self.size) as i32,
                true,
                true
            );
            frame.set_image(Some(image));
        };
        page.end();
        page.set_pad(PAD);
        page.set_margin(PAD);
        page.fixed(&header, HEIGHT);
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::Size(value) => {
                self.size = value;
            }
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
                    self.images.push(file);
                };
            }
            self.images.sort();
            self.current = 0;
        };
    }

    fn prev(&mut self) {
        if !self.images.is_empty() {
            self.current = match self.current > 0 {
                true => self.current.saturating_sub(1),
                false => self.images.len() - 1,
            };
        }
    }

    fn next(&mut self) {
        if !self.images.is_empty() {
            self.current = match self.current < self.images.len() - 1 {
                true => self.current.saturating_add(1),
                false => 0,
            };
        }
    }

    fn remove(&mut self) {
        if !self.images.is_empty() {
            match choice2_default("Remove ...?", "Remove", "Cancel", "Permanent") {
                Some(0) => {
                    self.images.remove(self.current);
                }
                Some(2) => {
                    if fs::remove_file(self.images[self.current].clone()).is_ok() {
                        self.images.remove(self.current);
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

fn frame(tooltip: &str) -> Frame {
    let mut element = Frame::default_fill();
    element.set_tooltip(tooltip);
    element.set_image(None::<SharedImage>);
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
            |_|Message::Open,
        )
        .on_item_event(
            "&File/@#1+  &Remove",
            Shortcut::Ctrl | 'd',
            MenuFlag::Normal,
            |_|Message::Remove,
        )
        .on_item_event(
            "&Image/@#>|  &Next",
            Shortcut::Ctrl | 'n',
            MenuFlag::Normal,
            |_|Message::Next,
        )
        .on_item_event(
            "&Image/@#|<  &Prev",
            Shortcut::Ctrl | 'p',
            MenuFlag::Normal,
            |_|Message::Prev,
        )
        .on_item_event(
            "@#1+  &Quit",
            Shortcut::Ctrl | 'q',
            MenuFlag::Normal,
            |_|Message::Quit,
        );
}


fn slider(tooltip: &str, sz: f64) -> Slider {
    let mut element = Slider::default();
    element.set_tooltip(tooltip);
    element.set_value(sz);
    element
}
