#![forbid(unsafe_code)]

use flemish::{
    app,
    browser::{Browser, BrowserType},
    button::Button,
    color_themes,
    enums::{CallbackTrigger, Font, FrameType},
    frame::Frame,
    group::Flex,
    prelude::*,
    OnEvent, Sandbox, Settings,
};

#[derive(Debug, Clone)]
struct Model {
    list: Vec<char>,
    curr: usize,
}

impl Model {
    fn init() -> Self {
        Self {
            list: (0x2700..=0x27BF)
                .map(|x| char::from_u32(x).unwrap())
                .collect(),
            curr: 0,
        }
    }
    fn choice(&mut self, curr: usize) {
        self.curr = curr;
    }
    pub fn inc(&mut self) {
        if !self.list.is_empty() {
            match self.curr < self.list.len() - 1 {
                true => self.curr += 1,
                false => self.curr = 0,
            };
        }
    }
    pub fn dec(&mut self) {
        if !self.list.is_empty() {
            match self.curr > 0 {
                true => self.curr -= 1,
                false => self.curr = self.list.len() - 1,
            };
        }
    }
}

pub fn main() {
    Model::new().run(Settings {
        size: (640, 360),
        resizable: false,
        ignore_esc_close: true,
        color_map: Some(color_themes::DARK_THEME),
        scheme: Some(app::Scheme::Base),
        ..Default::default()
    })
}

const PAD: i32 = 10;
const HEIGHT: i32 = PAD * 3;
const WIDTH: i32 = HEIGHT * 3;

#[derive(Clone, Copy)]
enum Message {
    Inc,
    Dec,
    Choice(usize),
}

impl Sandbox for Model {
    type Message = Message;

    fn new() -> Self {
        Model::init()
    }

    fn title(&self) -> String {
        String::from("FlPictures")
    }

    fn view(&mut self) {
        let mut page = Flex::default_fill();
        {
            let mut left = Flex::default_fill().column();
            crate::browser("List", self.clone()).on_event(move |browser| {
                Message::Choice((browser.value() as usize).saturating_sub(1))
            });
            let mut buttons = Flex::default();
            Button::default()
                .with_label("@#<")
                .on_event(move |_| Message::Dec);
            Button::default()
                .with_label("@#>")
                .on_event(move |_| Message::Inc);
            buttons.end();
            buttons.set_pad(0);
            left.end();
            left.set_pad(PAD);
            left.fixed(&buttons, HEIGHT);
            page.fixed(&left, WIDTH);
            crate::frame("Canvas", self.clone());
        }
        page.end();
        page.set_pad(PAD);
        page.set_margin(PAD);
        page.set_frame(FrameType::FlatBox);
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::Choice(value) => self.choice(value),
            Message::Dec => self.dec(),
            Message::Inc => self.inc(),
        }
    }
}

fn browser(tooltip: &str, value: Model) -> Browser {
    let mut element = Browser::default().with_type(BrowserType::Hold);
    element.set_trigger(CallbackTrigger::Changed);
    element.set_tooltip(tooltip);
    element.set_label_font(Font::Zapfdingbats);
    element.set_text_size(16);
    if !value.list.is_empty() {
        for item in value.list {
            element.add(&item.to_string());
        }
        element.select(value.curr as i32 + 1);
        element.top_line(value.curr as i32 + 1);
    }
    element
}

fn frame(tooltip: &str, value: Model) -> Frame {
    let mut element = Frame::default();
    element.set_frame(FrameType::DownBox);
    element.set_label_font(Font::Zapfdingbats);
    element.set_label_size(250);
    element.set_tooltip(tooltip);
    if !value.list.is_empty() {
        element.set_label(&value.list[value.curr].to_string());
    };
    element
}
