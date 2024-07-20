mod model;

use {
    flemish::{
        app,
        button::Button,
        color_themes,
        enums::{Color, Font},
        frame::Frame,
        group::Flex,
        prelude::*,
        text::{TextBuffer, TextEditor, WrapMode},
        OnEvent, Sandbox, Settings,
    },
    model::Model,
};

const PAD: i32 = 10;
const HEIGHT: i32 = PAD * 3;

#[derive(Clone)]
pub enum Message {
    Encode,
    Decode,
    Source(String),
    Target(String),
}

fn main() {
    Model::new().run(Settings {
        size: (640, 360),
        ignore_esc_close: true,
        resizable: false,
        color_map: Some(color_themes::DARK_THEME),
        scheme: Some(app::Scheme::Base),
        ..Default::default()
    })
}

impl Sandbox for Model {
    type Message = Message;

    fn new() -> Self {
        Self::default()
    }

    fn title(&self) -> String {
        String::from("FlBase64")
    }

    fn view(&mut self) {
        let mut page = Flex::default_fill().column();
        {
            page.fixed(&Frame::default(), HEIGHT);
            let mut hero = Flex::default();
            {
                crate::texteditor("Normal text", &self.decode, self.font, self.size)
                    .on_event(move |text| Message::Source(text.buffer().unwrap().text()));
                crate::texteditor("Base64 text", &self.encode, self.font, self.size)
                    .on_event(move |text| Message::Target(text.buffer().unwrap().text()));
            }
            hero.end();
            hero.set_pad(PAD);
            let mut footer = Flex::default();
            {
                crate::button("Decode", "@<-", &mut footer).on_event(move |_| Message::Decode);
                Frame::default();
                crate::button("Encode", "@->", &mut footer).on_event(move |_| Message::Encode);
            }
            footer.end();
            page.fixed(&footer, HEIGHT);
        }
        page.end();
        page.set_pad(PAD);
        page.set_margin(PAD);
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::Source(value) => self.decode = value,
            Message::Target(value) => self.encode = value,
            Message::Encode => self.encode(),
            Message::Decode => self.decode(),
        }
    }
}

fn texteditor(tooltip: &str, value: &str, font: i32, size: i32) -> TextEditor {
    let mut element = TextEditor::default().with_label(tooltip);
    element.set_tooltip(tooltip);
    element.set_linenumber_width(0);
    element.set_buffer(TextBuffer::default());
    element.wrap_mode(WrapMode::AtBounds, 0);
    element.buffer().unwrap().set_text(value);
    element.set_color(Color::from_hex(0x002b36));
    element.set_text_color(Color::from_hex(0x93a1a1));
    element.set_text_font(Font::by_index(font as usize));
    element.set_text_size(size);
    element
}

fn button(tooltip: &str, label: &str, flex: &mut Flex) -> Button {
    let mut element = Button::default().with_label(label);
    element.set_tooltip(tooltip);
    element.set_label_size(HEIGHT / 2);
    flex.fixed(&element, HEIGHT);
    element
}
