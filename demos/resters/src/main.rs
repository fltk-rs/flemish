#![forbid(unsafe_code)]

mod model;

use {
    flemish::{
        app,
        button::Button,
        color_themes,
        enums::{Align, Color, Event, Font, FrameType},
        frame::Frame,
        misc::InputChoice,
        group::Flex,
        menu::Choice,
        prelude::*,
        text::{StyleTableEntry, TextBuffer, TextDisplay, WrapMode},
        OnEvent, Sandbox, Settings,
        valuator::Dial,
    },
    std::{process::Command, thread},
    model::Model,
};

const SPINNER: Event = Event::from_i32(405);

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
        String::from("flResters")
    }

    fn view(&mut self) {
        let mut page = Flex::default_fill().column();
        let mut header = Flex::default();
        header.fixed(&Frame::default(), WIDTH);
        crate::choice(self.method as i32, &mut header).with_label("Method: ")
            .on_event(move |choice| Message::Method(choice.value() as u8));
        header.fixed(&Frame::default(), WIDTH);
        crate::input(&self.url).on_event(move |input| Message::Url(input.value().unwrap()));
        crate::button(&mut header).on_event(move |_| Message::Thread);
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
        footer.fixed(&crate::dial(), HEIGHT);
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
                let handler = thread::spawn(move || -> (bool, String) { crate::curl(clone) });
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

fn dial() -> Dial {
    const MAX: u8 = 120;
    let mut element = Dial::default();
    // element.deactivate();
    element.set_maximum((MAX / 4 * 3) as f64);
    element.set_value(element.minimum());
    element.handle(move |dial, event| {
        if event == crate::SPINNER {
            dial.set_value(if dial.value() == (MAX - 1) as f64 {
                dial.minimum()
            } else {
                dial.value() + 1f64
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
    buffer.set_text(&model::fill_style_buffer(value));
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
    element.add(r#"https:\/\/lingva.ml\/api\/v1\/languages"#);
    element.add(r#"https:\/\/ipinfo.io\/json"#);
    element.set_value(value);
    element
}

const PAD: i32 = 10;
const HEIGHT: i32 = PAD * 3;
const WIDTH: i32 = HEIGHT * 3;
