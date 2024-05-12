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
    valuator::{Slider, SliderType},
    OnEvent, OnMenuEvent, Sandbox, Settings,
};
use std::fs;

pub fn main() {
    Model::new().run(Settings {
        size: (640, 480),
        resizable: true,
        ignore_esc_close: true,
        color_map: Some(color_themes::DARK_THEME),
        scheme: Some(app::Scheme::Base),
        ..Default::default()
    })
}

const PAD: i32 = 10;
const HEIGHT: i32 = PAD * 3;

struct Image {
    file: String,
    image: SharedImage,
}

struct Model {
    list: Vec<Image>,
    size: f64,
    current: usize,
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
        crate::button("Open", "@#fileopen", &mut header).on_event(|_| Message::Open);
        crate::button("Prev", "@#|<", &mut header).on_event(|_| Message::Prev);
        let mut size = crate::slider("Size").with_type(SliderType::Horizontal);
        crate::button("Next", "@#>|", &mut header).on_event(|_| Message::Next);
        crate::button("Remove", "@#1+", &mut header).on_event(|_| Message::Remove);
        header.end();
        let mut hero = Flex::default_fill();
        let mut frame = crate::frame("Image").with_id("image-frame");
        hero.end();
        page.end();
        {
            header.set_pad(PAD);
            header.set_margin(PAD);
            hero.set_pad(PAD);
            hero.set_margin(PAD);
            hero.set_frame(FrameType::DownBox);
            page.set_frame(FrameType::FlatBox);
            page.set_pad(0);
            page.set_margin(0);
            page.fixed(&header, HEIGHT + PAD * 2);
        }

        let image = if self.list.is_empty() {
            None::<SharedImage>
        } else {
            let mut image = self.list[self.current].image.clone();
            image.scale(
                (frame.w() as f64 * self.size) as i32,
                (frame.h() as f64 * self.size) as i32,
                true,
                true,
            );
            Some(image)
        };

        frame.set_image(image.clone());
        size.set_callback(move |s| slider_cb(s, image.clone()));
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
            self.current = 0;
        };
    }

    fn prev(&mut self) {
        if !self.list.is_empty() {
            self.current = match self.current > 0 {
                true => self.current.saturating_sub(1),
                false => self.list.len() - 1,
            };
        }
    }

    fn next(&mut self) {
        if !self.list.is_empty() {
            self.current = match self.current < self.list.len() - 1 {
                true => self.current.saturating_add(1),
                false => 0,
            };
        }
    }

    fn remove(&mut self) {
        if !self.list.is_empty() {
            match choice2_default("Remove ...?", "Remove", "Cancel", "Permanent") {
                Some(0) => {
                    self.list.remove(self.current);
                }
                Some(2) => {
                    if fs::remove_file(self.list[self.current].file.clone()).is_ok() {
                        self.list.remove(self.current);
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

fn slider(tooltip: &str) -> Slider {
    let mut element = Slider::default();
    element.set_tooltip(tooltip);
    element.set_value(element.maximum());
    element
}

fn slider_cb(s: &mut Slider, image: Option<SharedImage>) {
    let mut frame: Frame = app::widget_from_id("image-frame").unwrap();
    if let Some(mut image) = image.clone() {
        image.scale(
            (frame.width() as f64 * s.value()) as i32,
            (frame.height() as f64 * s.value()) as i32,
            true,
            true,
        );
        frame.set_image(Some(image));
        app::redraw();
    }
}
