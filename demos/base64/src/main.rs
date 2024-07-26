mod model;

use {
    flemish::{
        button::Button,
        color_themes,
        enums::{Color, Event, Font},
        frame::Frame,
        image::SvgImage,
        group::{Flex, FlexType},
        prelude::*,
        text::{TextBuffer, TextEditor, WrapMode},
        OnEvent, Sandbox, Settings,
    },
    model::Model,
};

const PAD: i32 = 10;
const HEIGHT: i32 = PAD * 3;
const NAME: &str = "FlBase64";

#[derive(Clone)]
pub enum Message {
    Encode,
    Decode,
    Source(String),
    Target(String),
}

fn main() {
    Model::new().run(Settings {
        resizable: true,
        size: (360, 640),
        xclass: Some(String::from(NAME)),
        icon: Some(SvgImage::from_data(include_str!("../../assets/logo.svg")).unwrap()),
        color_map: Some(color_themes::DARK_THEME),
        ..Default::default()
    })
}

impl Sandbox for Model {
    type Message = Message;

    fn new() -> Self {
        Self::default()
    }

    fn title(&self) -> String {
        String::from(NAME)
    }

    fn view(&mut self) {
        let mut page = Flex::default_fill().column();
        {
            let mut hero = Flex::default_fill();
            {
                crate::texteditor("Normal text", &self.decode, self.font, self.size)
                    .on_event(move |text| Message::Source(text.buffer().unwrap().text()));
                Frame::default();
                crate::texteditor("Base64 text", &self.encode, self.font, self.size)
                    .on_event(move |text| Message::Target(text.buffer().unwrap().text()));
            }
            hero.end();
            hero.set_pad(0);
            crate::orientation(&mut hero);
            hero.handle(crate::resize);
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
    let mut element = TextEditor::default();
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

fn resize(flex: &mut Flex, event: Event) -> bool {
    if event == Event::Resize {
        crate::orientation(flex);
        flex.fixed(&flex.child(1).unwrap(), PAD);
        true
    } else {
        false
    }
}

fn orientation(flex: &mut Flex) {
    flex.set_type(match flex.width() < flex.height() {
        true => FlexType::Column,
        false => FlexType::Row,
    });
}
