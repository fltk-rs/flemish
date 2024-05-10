#![forbid(unsafe_code)]

use flemish::{
    app, button::Button, color_themes, frame::Frame, group::Flex, prelude::*, OnEvent, Sandbox,
    Settings,
};

pub fn main() {
    MyApp::new().run(Settings {
        size: (300, 180),
        resizable: true,
        ignore_esc_close: true,
        color_map: Some(color_themes::BLACK_THEME),
        scheme: Some(app::Scheme::Base),
        ..Default::default()
    })
}

#[derive(Debug, Clone)]
enum MyAppMessage {
    PageA(PageAMessage),
    PageB(PageBMessage),
}

trait Page {
    fn update(&mut self, message: MyAppMessage) -> Option<Box<dyn Page>>;
    fn view(&self);
}

struct MyApp {
    page: Box<dyn Page>,
}
impl Sandbox for MyApp {
    type Message = MyAppMessage;

    fn new() -> Self {
        Self {
            page: Box::new(PageA::new()),
        }
    }

    fn title(&self) -> String {
        String::from("My App")
    }

    fn update(&mut self, message: Self::Message) {
        let page = self.page.update(message);
        if let Some(p) = page {
            self.page = p;
        }
    }

    fn view(&mut self) {
        self.page.view();
    }
}

#[derive(Debug, Clone)]
enum PageBMessage {
    ButtonPressed,
}
type Mb = PageBMessage;

struct PageB;

impl PageB {
    fn new() -> Self {
        Self
    }
}

impl Page for PageB {
    fn update(&mut self, message: MyAppMessage) -> Option<Box<dyn Page>> {
        if let MyAppMessage::PageB(msg) = message {
            match msg {
                PageBMessage::ButtonPressed => return Some(Box::new(PageA::new())),
            }
        }
        None
    }

    fn view(&self) {
        let col = Flex::default_fill().column();
        Frame::default().with_label("Hello!");
        Button::default()
            .with_label("Log out")
            .on_event(|_| MyAppMessage::PageB(Mb::ButtonPressed));
        col.end();
    }
}

#[derive(Debug, Clone)]
enum PageAMessage {
    TextChanged(String),
}
type Ma = PageAMessage;

struct PageA {
    password: String,
}

impl PageA {
    fn new() -> Self {
        Self {
            password: String::new(),
        }
    }
}

impl Page for PageA {
    fn update(&mut self, message: MyAppMessage) -> Option<Box<dyn Page>> {
        if let MyAppMessage::PageA(msg) = message {
            match msg {
                PageAMessage::TextChanged(s) => {
                    self.password = s;
                    if self.password == "abc" {
                        return Some(Box::new(PageB::new()));
                    }
                }
            }
        }
        None
    }

    fn view(&self) {
        let col = Flex::default_fill().column();
        let mut i = flemish::input::SecretInput::default();
        i.set_value(&self.password);
        Button::default()
            .with_label("Log in")
            .on_event(move |_| MyAppMessage::PageA(Ma::TextChanged(i.value())));
        col.end();
    }
}
