mod model;

use {
    flemish::{
        app,
        color_themes,
        enums::{Color,Shortcut},
        image::SvgImage,
        group::Flex,
        dialog::{FileChooser,FileChooserType},
        menu::{MenuBar, MenuFlag},
        prelude::*,
        text::{TextBuffer, TextEditor, WrapMode},
        OnEvent, OnMenuEvent, Sandbox, Settings,
    },
    model::Model,
    std::env,
};

const PAD: i32 = 10;
const HEIGHT: i32 = PAD * 3;
const NAME: &str = "FlText";

#[derive(Clone)]
pub enum Message {
    Open,
    Save,
    Change,
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
            page.fixed(&crate::menu(), HEIGHT);
            crate::texteditor(&self.text).on_event(move |_| Message::Change);
        }
        page.end();
        page.set_pad(PAD);
        page.set_margin(PAD);
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::Change => self.save = false,
            Message::Open => {
                let mut dialog = FileChooser::new(
                    env::var("HOME").unwrap(),
                    "*.{txt,md}",
                    FileChooserType::Single,
                    "Open ...",
                );
                dialog.show();
                while dialog.shown() {
                    app::wait();
                }
                if dialog.count() > 0 {
                    if let Some(file) = dialog.value(1) {
                        self.path = file;
                        self.open();
                        self.save = true;
                    };
                };
            }
            Message::Save => {
                if !self.text.is_empty() {
                    let mut dialog = FileChooser::new(
                        std::env::var("HOME").unwrap(),
                        "*.{txt,md}",
                        FileChooserType::Create,
                        "Save ...",
                    );
                    dialog.show();
                    while dialog.shown() {
                        app::wait();
                    }
                    if dialog.count() > 0 {
                        if let Some(file) = dialog.value(1) {
                            self.path = file
                        };
                    };
                    self.save();
                }
            }
        }
    }
}

fn menu() -> MenuBar {
    MenuBar::default()
        .clone()
        .on_item_event(
            "@#fileopen  &Open...",
            Shortcut::Ctrl | 'o',
            MenuFlag::Normal,
            move |_| Message::Open,
        )
        .on_item_event(
            "@#filesaveas  &Save as...",
            Shortcut::Ctrl | 's',
            MenuFlag::Normal,
            move |_| Message::Save,
        )
}

fn texteditor(value: &str) -> TextEditor {
    let mut element = TextEditor::default();
    element.set_linenumber_width(0);
    element.set_buffer(TextBuffer::default());
    element.wrap_mode(WrapMode::AtBounds, 0);
    element.buffer().unwrap().set_text(value);
    element.set_color(Color::from_hex(0x002b36));
    element.set_text_color(Color::from_hex(0x93a1a1));
    element
}

