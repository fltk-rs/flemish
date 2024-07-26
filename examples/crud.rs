#![forbid(unsafe_code)]

use flemish::{
    app,
    browser::{Browser, BrowserType},
    button::Button,
    color_themes,
    frame::Frame,
    group::Flex,
    input::Input,
    prelude::*,
    OnEvent, Sandbox, Settings,
};

pub fn main() {
    Model::new().run(Settings {
        size: (640, 360),
        color_map: Some(color_themes::DARK_THEME),
        ..Default::default()
    })
}

const PAD: i32 = 10;
const HEIGHT: i32 = PAD * 3;
const WIDTH: i32 = HEIGHT * 3;

#[derive(Debug, Clone)]
enum Message {
    Inc,
    Dec,
    Upd,
    Choice(usize),
    Filter(String),
    Name(String),
    Surname(String),
    Secretname(String),
}

#[derive(Debug, Clone)]
struct Person {
    name: String,
    surname: String,
    secretname: String,
}

impl Person {
    fn default() -> Self {
        Self {
            name: String::new(),
            surname: String::new(),
            secretname: String::new(),
        }
    }
}

#[derive(Debug, Clone)]
struct Model {
    filter: String,
    inc: bool,
    dec: bool,
    upd: bool,
    list: Vec<Person>,
    temp: Person,
    curr: usize,
}

impl Model {
    fn default() -> Self {
        Self {
            filter: String::new(),
            inc: true,
            dec: false,
            upd: false,
            list: Vec::from([Person::default()]),
            temp: Person::default(),
            curr: 0,
        }
    }
    fn inc(&mut self) {
        self.list.push(self.temp.clone());
        self.curr = self.curr.saturating_add(1);
        self.temp = Person::default();
    }
    fn dec(&mut self) {
        self.list.remove(self.curr);
        self.curr = self.curr.saturating_sub(1);
        self.temp = self.list[self.curr].clone();
    }
    fn upd(&mut self) {
        self.list[self.curr] = self.temp.clone();
    }
    fn name(&mut self, value: String) {
        self.temp.name = value;
    }
    fn sur(&mut self, value: String) {
        self.temp.surname = value;
    }
    fn sec(&mut self, value: String) {
        self.temp.secretname = value;
    }
}

impl Sandbox for Model {
    type Message = Message;

    fn new() -> Self {
        Self::default()
    }

    fn title(&self) -> String {
        String::from("7GUI: CRUD")
    }

    fn view(&mut self) {
        let mut page = Flex::default_fill();
        {
            let mut left = Flex::default_fill().column();

            let mut row = Flex::default();
            row.fixed(&Frame::default(), HEIGHT);
            crate::input(&self.filter)
                .with_label("@#search")
                .on_event(move |input| Message::Filter(input.value()));
            row.end();
            left.fixed(&row, HEIGHT);

            crate::browser("Browser", self).on_event(move |browser| {
                Message::Choice((browser.value() as usize).saturating_sub(1))
            });

            let mut row = Flex::default();
            crate::button(self.inc)
                .with_label("@#+  Add")
                .on_event(move |_| Message::Inc);
            crate::button(self.upd)
                .with_label("@#refresh  Update")
                .on_event(move |_| Message::Upd);
            crate::button(self.dec)
                .with_label("@#1+  Delete")
                .on_event(move |_| Message::Dec);
            row.end();
            row.set_pad(PAD);
            left.fixed(&row, HEIGHT);

            left.end();
            left.set_pad(PAD);
            page.fixed(&Frame::default(), WIDTH);

            let mut right = Flex::default_fill().column();
            right.fixed(
                &crate::input(&self.temp.name)
                    .with_label("Name:")
                    .clone()
                    .on_event(move |input| Message::Name(input.value())),
                HEIGHT,
            );
            right.fixed(
                &crate::input(&self.temp.surname)
                    .with_label("Surname:")
                    .clone()
                    .on_event(move |input| Message::Surname(input.value())),
                HEIGHT,
            );
            right.fixed(
                &crate::input(&self.temp.secretname)
                    .with_label("Secretname:")
                    .clone()
                    .on_event(move |input| Message::Secretname(input.value())),
                HEIGHT,
            );
            right.end();
        }
        page.end();
        page.set_pad(0);
        page.set_margin(10);
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::Choice(value) => {
                self.curr = value;
                self.upd = true;
                if self.list.len() > 1 {
                    self.dec = true;
                }
            }
            Message::Filter(value) => self.filter = value,
            Message::Name(value) => self.name(value),
            Message::Surname(value) => self.sur(value),
            Message::Secretname(value) => self.sec(value),
            Message::Inc => self.inc(),
            Message::Upd => self.upd(),
            Message::Dec => self.dec(),
        }
    }
}

fn button(value: bool) -> Button {
    let mut element = Button::default();
    if value {
        element.activate();
    } else {
        element.deactivate();
    }
    element
}

fn browser(tooltip: &str, value: &Model) -> Browser {
    let mut element = Browser::default().with_type(BrowserType::Hold);
    element.set_tooltip(tooltip);
    if !value.list.is_empty() {
        for item in &value.list {
            if item
                .name
                .to_lowercase()
                .starts_with(&value.filter.to_lowercase())
            {
                element.add(&format!(
                    "{} {} {}",
                    item.name, item.surname, item.secretname
                ));
            }
        }
        element.select(value.curr as i32 + 1);
    }
    element
}

fn input(value: &str) -> Input {
    let mut element = Input::default();
    element.set_value(value);
    element
}
