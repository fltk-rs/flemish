#![forbid(unsafe_code)]

mod model;

use {
    flemish::{
        app,
        button::Button,
        color_themes,
        enums::{Color, Event, Font, FrameType},
        frame::Frame,
        group::Flex,
        menu::Choice,
        misc::{InputChoice,Progress},
        prelude::*,
        text::{StyleTableEntry, TextBuffer, TextDisplay, WrapMode},
        OnEvent, Sandbox, Settings,
    },
    json_tools::{Buffer, BufferType, Lexer, Span, TokenType},
    model::Model,
};

fn main() {
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
    Method(u8),
    Url(String),
    Thread,
}

impl Sandbox for Model {
    type Message = Message;

    fn new() -> Self {
        Model::default()
    }

    fn title(&self) -> String {
        String::from("FlResters")
    }

    fn view(&mut self) {
        let mut page = Flex::default_fill().column();
        let mut header = Flex::default();
        header.fixed(&Frame::default(), WIDTH);
        crate::choice(self.method as i32, &mut header)
            .with_label("Method: ")
            .on_event(move |choice| Message::Method(choice.value() as u8));
        header.fixed(&Frame::default(), WIDTH);
        crate::input(&self.url).on_event(move |input| Message::Url(input.value().unwrap()));
        crate::button(&mut header).on_event(move |_| Message::Thread);
        header.end();
        crate::text(&self.responce);
        let mut footer = Flex::default();
        footer.fixed(&Frame::default().with_label("Status: "), WIDTH);
        Frame::default();
        footer.fixed(&crate::progress().with_label(&self.status), WIDTH);
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
            Message::Thread => {
                let clone = self.clone();
                let handler = std::thread::spawn(move || -> (bool, String) { crate::curl(clone) });
                while !handler.is_finished() {
                    app::wait();
                    app::handle_main(SPINNER).unwrap();
                    app::sleep(0.02);
                }
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

fn curl(model: Model) -> (bool, String) {
    let url = match model.url.starts_with("https://") {
        true => model.url.clone(),
        false => String::from("https://") + &model.url,
    };
    if let Ok(response) = match model.method {
        0 => ureq::get(&url),
        1 => ureq::post(&url),
        _ => unreachable!(),
    }
    .call()
    {
        (true, response.into_string().unwrap())
    } else {
        (false, String::from("Error"))
    }
}

fn progress() -> Progress {
    const MAX: u8 = 120;
    let mut element = Progress::default();
    element.set_maximum((MAX / 4 * 3) as f64);
    element.set_value(element.minimum());
    element.handle(move |progress, event| {
        if event == crate::SPINNER {
            progress.set_value(if progress.value() == (MAX - 1) as f64 {
                progress.minimum()
            } else {
                progress.value() + 1f64
            });
            true
        } else {
            false
        }
    });
    element
}

fn choice(value: i32, flex: &mut Flex) -> Choice {
    let mut element = Choice::default();
    element.add_choice("GET|POST");
    element.set_value(value);
    flex.fixed(&element, WIDTH);
    element
}

fn text(value: &str) {
    let mut buffer = TextBuffer::default();
    buffer.set_text(&crate::fill_style_buffer(value));
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
    element.set_highlight_data(buffer, styles);
    element.buffer().unwrap().set_text(value);
}

fn button(flex: &mut Flex) -> Button {
    let mut element = Button::default().with_label("@#search");
    element.set_label_size(18);
    flex.fixed(&element, HEIGHT);
    element
}

fn input(value: &str) -> InputChoice {
    let mut element = InputChoice::default().with_label("URL: ");
    for item in ["users", "posts", "albums", "todos", "comments", "posts"] {
        element.add(&(format!(r#"https:\/\/jsonplaceholder.typicode.com\/{item}"#)));
    }
    element.add(r#"https:\/\/lingva.thedaviddelta.com\/api\/v1\/languages"#);
    element.add(r#"https:\/\/ipinfo.io\/json"#);
    element.set_value(value);
    element
}

pub fn fill_style_buffer(s: &str) -> String {
    let mut buffer = vec![b'A'; s.len()];
    for token in Lexer::new(s.bytes(), BufferType::Span) {
        let c = match token.kind {
            TokenType::CurlyOpen
            | TokenType::CurlyClose
            | TokenType::BracketOpen
            | TokenType::BracketClose
            | TokenType::Colon
            | TokenType::Comma
            | TokenType::Invalid => 'A',
            TokenType::String => 'B',
            TokenType::BooleanTrue | TokenType::BooleanFalse | TokenType::Null => 'C',
            TokenType::Number => 'D',
        };
        if let Buffer::Span(Span { first, end }) = token.buf {
            let start = first as _;
            let last = end as _;
            buffer[start..last].copy_from_slice(c.to_string().repeat(last - start).as_bytes());
        }
    }
    String::from_utf8_lossy(&buffer).to_string()
}

const SPINNER: Event = Event::from_i32(405);
const PAD: i32 = 10;
const HEIGHT: i32 = PAD * 3;
const WIDTH: i32 = HEIGHT * 3;
