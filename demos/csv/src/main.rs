#![forbid(unsafe_code)]

mod model;

use {
    flemish::{
        app,
        browser::{Browser, BrowserType},
        button::Button,
        color_themes, draw,
        enums::{Color, FrameType},
        frame::Frame,
        group::Flex,
        prelude::*,
        OnEvent, Sandbox, Settings,
    },
    model::Model,
};

const NAME: &str = "FlCSV";
const PAD: i32 = 10;
const HEIGHT: i32 = PAD * 3;
const WIDTH: i32 = HEIGHT * 3;

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

#[derive(Clone)]
pub enum Message {
    Choice(usize),
    Save,
}

impl Sandbox for Model {
    type Message = Message;

    fn title(&self) -> String {
        String::from(NAME)
    }

    fn new() -> Self {
        let mut model = Self::default();
        model.init();
        model
    }

    fn view(&mut self) {
        let mut page = Flex::default_fill();
        {
            let mut left = Flex::default_fill().column();
            crate::browser("Browser", self.temp.clone(), self.curr).on_event(move |browser| {
                Message::Choice((browser.value() as usize).saturating_sub(1))
            });
            crate::button("Save image", &mut left).on_event(move |_| Message::Save);
            left.end();
            left.set_pad(PAD);
            page.fixed(&left, WIDTH);
        }
        crate::frame("Canvas", self.clone());
        page.end();
        page.set_pad(PAD);
        page.set_margin(PAD);
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::Save => {}
            Message::Choice(value) => {
                println!("!");
                self.choice(value);
            }
        }
    }
}

fn browser(tooltip: &str, temp: Vec<String>, curr: usize) -> Browser {
    let mut element = Browser::default().with_type(BrowserType::Hold);
    element.set_tooltip(tooltip);
    if !temp.is_empty() {
        for item in temp {
            element.add(&item);
        }
        element.select(curr as i32 + 1);
    }
    element
}

fn button(tooltip: &str, flex: &mut Flex) -> Button {
    let mut element = Button::default().with_label("@#filesave");
    element.set_tooltip(tooltip);
    flex.fixed(&element, HEIGHT);
    element
}

fn frame(tooltip: &str, value: Model) {
    let mut element = Frame::default();
    element.set_frame(FrameType::DownBox);
    element.set_tooltip(tooltip);
    element.set_color(Color::Black);
    element.draw(move |frame| {
        if let Some(data) = value.cash.get(&value.temp[value.curr]) {
            let mut highest = data
                .iter()
                .map(|elem| elem.low)
                .collect::<Vec<f64>>()
                .iter()
                .cloned()
                .fold(f64::NAN, f64::max);
            highest += (highest.to_string().len() * 10) as f64 / 3.;
            let factor = frame.h() as f64 / highest;
            if !data.is_empty() {
                let step = frame.w() / data.len() as i32;
                let mut idx = frame.x() + step;
                for elem in data {
                    let open = frame.h() - (elem.open * factor) as i32;
                    let high = frame.h() - (elem.high * factor) as i32;
                    let low = frame.h() - (elem.low * factor) as i32;
                    let close = frame.h() - (elem.close * factor) as i32;
                    draw::draw_line(idx, high, idx, low);
                    let col = if close > open {
                        Color::Red
                    } else {
                        Color::Green
                    };
                    draw::set_draw_color(col);
                    draw::draw_rectf(idx - 2, open, 4, i32::abs(close - open));
                    draw::set_draw_color(Color::White);
                    idx += step;
                }
            };
        }
    });
}
