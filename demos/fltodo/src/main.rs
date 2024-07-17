#![forbid(unsafe_code)]

mod model;

use {
    flemish::{
        app,
        button::{Button, CheckButton},
        color_themes,
        enums::{FrameType, Shortcut},
        frame::Frame,
        group::{Flex, Scroll, ScrollType},
        input::Input,
        menu::{MenuButton, MenuFlag},
        prelude::*,
        OnEvent, OnMenuEvent, Sandbox, Settings,
    },
    model::Model,
    std::{env, fs},
};

pub fn main() {
    app::GlobalState::<String>::new(env::var("HOME").unwrap() + "/.config/" + NAME);
    Model::new().run(Settings {
        size: (360, 640),
        resizable: false,
        ignore_esc_close: true,
        color_map: Some(color_themes::DARK_THEME),
        scheme: Some(app::Scheme::Base),
        ..Default::default()
    })
}

const NAME: &str = "FlTodo";
const PAD: i32 = 10;
const HEIGHT: i32 = PAD * 3;

#[derive(Clone)]
pub enum Message {
    New(String),
    Delete(usize),
    Check(usize),
    Change((usize, String)),
    Quit,
}

impl Sandbox for Model {
    type Message = Message;

    fn title(&self) -> String {
        String::from(NAME)
    }

    fn new() -> Self {
        let file = app::GlobalState::<String>::get().with(move |file| file.clone());
        let default = Self { tasks: Vec::new() };
        if let Ok(value) = fs::read(file) {
            if let Ok(value) = rmp_serde::from_slice(&value) {
                value
            } else {
                default
            }
        } else {
            default
        }
    }

    fn view(&mut self) {
        let mut page = Flex::default_fill().column();
        let mut header = Flex::default(); // HEADER
        header.fixed(&crate::menu(), 50);
        let description = Input::default();
        header.fixed(
            &Button::default()
                .with_label("@#+")
                .clone()
                .on_event(move |_| Message::New(description.value())),
            HEIGHT,
        );
        header.end();
        let scroll = Scroll::default()
            .with_size(324, 600)
            .with_type(ScrollType::Vertical);
        let mut hero = Flex::default_fill().column(); // HERO
        for (idx, task) in self.tasks.iter().enumerate() {
            let mut row = Flex::default();
            let mut status = CheckButton::default();
            status.set_value(task.status);
            row.fixed(&status, HEIGHT);
            status.on_event(move |_| Message::Check(idx));
            let mut description = Input::default();
            description.set_value(&task.description);
            match self.tasks[idx].status {
                true => description.deactivate(),
                false => description.activate(),
            }
            description.on_event(move |input| Message::Change((idx, input.value())));
            let delete = Button::default().with_label("@#1+");
            row.fixed(&delete, HEIGHT);
            delete.on_event(move |_| Message::Delete(idx));
            row.end();
            row.set_pad(0);
            hero.fixed(&row, HEIGHT)
        }
        hero.end();
        scroll.end();
        let footer = Flex::default();
        Frame::default().with_label(&format!("All tasks: {}", self.tasks.len()));
        footer.end(); // FOOTER
        page.end();
        {
            page.set_frame(FrameType::FlatBox);
            page.set_pad(PAD);
            page.set_margin(PAD);
            page.fixed(&header, HEIGHT);
            page.fixed(&footer, HEIGHT);
            for mut flex in [header, hero, footer] {
                flex.set_pad(0);
                flex.set_margin(0);
            }
        }
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::Quit => {
                self.save(app::GlobalState::<String>::get().with(move |file| file.clone()));
                app::quit();
            }
            Message::Delete(idx) => {
                self.tasks.remove(idx);
            }
            Message::Change((idx, value)) => self.tasks[idx].description = value,
            Message::Check(idx) => self.check(idx),
            Message::New(description) => self.add(description),
        }
    }
}

fn menu() -> MenuButton {
    let mut element = MenuButton::default().with_label("@#menu").on_item_event(
        "@#1+  &Quit",
        Shortcut::Ctrl | 'q',
        MenuFlag::Normal,
        |_| Message::Quit,
    );
    element.set_tooltip("Menu");
    element
}
