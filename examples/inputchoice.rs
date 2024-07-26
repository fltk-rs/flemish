#![forbid(unsafe_code)]

use flemish::{
    app, color_themes, frame::Frame, group::Flex, misc::InputChoice, prelude::*, OnEvent, Sandbox,
    Settings,
};

pub fn main() {
    Model::new().run(Settings {
        size: (360, 640),
        color_map: Some(color_themes::DARK_THEME),
        ..Default::default()
    })
}

const PAD: i32 = 10;

#[derive(Clone)]
enum Message {
    Selected(String),
}

struct Model {
    language: String,
    text: String,
}

impl Sandbox for Model {
    fn new() -> Self {
        Self {
            language: String::new(),
            text: String::new(),
        }
    }

    fn view(&mut self) {
        let mut page = Flex::default_fill().column();
        {
            Frame::default().with_label("What is your language?");
            crate::input(&self.language);
            Frame::default();
            Frame::default().with_label(&self.text);
        }
        page.end();
        page.fixed(&page.child(0).unwrap(), 30);
        page.fixed(&page.child(1).unwrap(), 30);
        page.fixed(&page.child(3).unwrap(), 30);
        page.set_margin(PAD);
    }

    type Message = Message;
    fn update(&mut self, message: Message) {
        match message {
            Message::Selected(value) => {
                self.language = value.clone();
                self.text = crate::hello(&value).to_string();
            }
        }
    }

    fn title(&self) -> String {
        format!("InputChoice -{}- Flemish", self.language)
    }
}

const LANGUAGES: [&str; 8] = [
    "Danish",
    "English",
    "French",
    "German",
    "Italian",
    "Portuguese",
    "Spanish",
    "Other",
];

fn hello(label: &str) -> &str {
    match label {
        "Danish" => "Halloy!",
        "English" => "Hello!",
        "French" => "Salut!",
        "German" => "Hallo!",
        "Italian" => "Ciao!",
        "Portuguese" => "Olá!",
        "Spanish" => "¡Hola!",
        _ => "... hello?",
    }
}

fn input(value: &str) {
    let mut element = InputChoice::default();
    element.input().set_value(value);
    let mut choice = element.clone();
    element.input().set_callback(move |input| {
        choice.clear();
        for lang in crate::LANGUAGES {
            if lang
                .to_lowercase()
                .starts_with(&input.value().to_lowercase())
            {
                choice.add(lang);
            }
        }
    });
    element.input().do_callback();
    element.set_value_index(0);
    element.on_event(move |choice| Message::Selected(choice.value().unwrap()));
}
