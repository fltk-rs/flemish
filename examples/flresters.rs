#![forbid(unsafe_code)]

use {
    flemish::{
        app,
        button::Button,
        color_themes,
        enums::{Align, Color, Font, FrameType},
        frame::Frame,
        group::Flex,
        input::Input,
        menu::Choice,
        prelude::*,
        text::{StyleTableEntry, TextBuffer, TextDisplay, WrapMode},
        OnEvent, Sandbox, Settings,
    },
    std::{process::Command, thread},
};

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
struct Model {
    method: u8,
    url: String,
    responce: String,
    status: String,
}

#[derive(Clone)]
enum Message {
    Method(u8),
    Url(String),
    Request,
}

impl Sandbox for Model {
    type Message = Message;

    fn new() -> Self {
        Self {
            method: 0,
            url: String::new(),
            responce: String::new(),
            status: String::new(),
        }
    }

    fn title(&self) -> String {
        String::from("flResters")
    }

    fn view(&mut self) {
        let mut page = Flex::default_fill().column();
        let mut header = Flex::default();
        crate::choice(self.method as i32, &mut header)
            .on_event(move |choice| Message::Method(choice.value() as u8));
        header.fixed(&Frame::default().with_label("https://"), WIDTH);
        crate::input(&self.url).on_event(move |input| Message::Url(input.value()));
        crate::button(&mut header).on_event(move |_| Message::Request);
        header.end();
        crate::text(&self.responce);
        let mut footer = Flex::default();
        footer.fixed(&Frame::default().with_label("Status: "), WIDTH);
        Frame::default();
        footer.fixed(
            &Frame::default()
                .with_align(Align::Left | Align::Inside)
                .with_label(&self.status),
            WIDTH,
        );
        footer.end();
        page.end();
        {
            header.set_pad(0);
            page.set_pad(PAD);
            page.set_margin(PAD);
            page.set_frame(FrameType::FlatBox);
            page.fixed(&header, HEIGHT);
            page.fixed(&footer, HEIGHT);
        }
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::Method(value) => self.method = value,
            Message::Url(value) => self.url = value,
            Message::Request => {
                let url = match self.url.starts_with("https://") {
                    true => self.url.clone(),
                    false => String::from("https://") + &self.url,
                };
                let handler = thread::spawn(move || -> (bool, String) { crate::curl(url) });
                if let Ok((status, check)) = handler.join() {
                    self.status = match status {
                        true => "OK",
                        false => "Fail",
                    }
                    .to_string();
                    self.responce = check;
                }
            }
        }
    }
}

fn choice(value: i32, flex: &mut Flex) -> Choice {
    let mut element = Choice::default();
    element.add_choice("GET|POST");
    element.set_value(value);
    flex.fixed(&element, WIDTH);
    element
}

fn text(value: &str) {
    let styles: Vec<StyleTableEntry> = [0xdc322f, 0x268bd2, 0x859900]
        .into_iter()
        .map(|color| StyleTableEntry {
            color: Color::from_hex(color),
            font: Font::Courier,
            size: 16,
        })
        .collect();
    let mut element = TextDisplay::default();
    element.wrap_mode(WrapMode::AtBounds, 0);
    element.set_buffer(TextBuffer::default());
    element.set_color(Color::from_hex(0x002b36));
    element.set_highlight_data(TextBuffer::default(), styles);
    element.buffer().unwrap().set_text(value);
}

fn button(flex: &mut Flex) -> Button {
    let mut element = Button::default().with_label("@#search");
    element.set_label_size(18);
    flex.fixed(&element, HEIGHT);
    element
}

fn input(value: &str) -> Input {
    let mut element = Input::default();
    element.set_value(value);
    element
}

fn curl(url: String) -> (bool, String) {
    let run = Command::new("curl")
        .args(["-s", &url])
        .output()
        .expect("failed to execute bash");
    (
        run.status.success(),
        String::from_utf8_lossy(match run.status.success() {
            true => &run.stdout,
            false => &run.stderr,
        })
        .to_string(),
    )
}

const PAD: i32 = 10;
const HEIGHT: i32 = PAD * 3;
const WIDTH: i32 = HEIGHT * 3;
