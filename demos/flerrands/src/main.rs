#![forbid(unsafe_code)]

use {
    flemish::{
        app,
        button::{Button, CheckButton},
        color_themes,
        enums::{FrameType, Shortcut},
        frame::Frame,
        group::Flex,
        input::Input,
        menu::{MenuButton, MenuFlag},
        prelude::*,
        OnEvent, OnMenuEvent, Sandbox, Settings,
    },
    serde::{Deserialize, Serialize},
    std::{env, fs},
};

pub fn main() {
    let mut app = Model::new();
    app.run(Settings {
        size: app.size,
        resizable: true,
        ignore_esc_close: true,
        color_map: Some(color_themes::DARK_THEME),
        scheme: Some(app::Scheme::Base),
        ..Default::default()
    })
}

const NAME: &str = "FlErrands";
const PATH: &str = "/.config/";
const PAD: i32 = 10;
const HEIGHT: i32 = PAD * 3;
const WINDOW_WIDTH: i32 = 360;
const WINDOW_HEIGHT: i32 = 640;

#[derive(Deserialize, Serialize)]
struct Task {
    status: bool,
    description: String,
}

#[derive(Deserialize, Serialize)]
struct Model {
    size: (i32, i32),
    tasks: Vec<Task>,
}

#[derive(Clone)]
enum Message {
    New(String),
    Delete(usize),
    Check(usize),
    Quit,
}

impl Sandbox for Model {
    type Message = Message;

    fn title(&self) -> String {
        String::from(NAME)
    }

    fn new() -> Self {
        let file = env::var("HOME").unwrap() + PATH + NAME;
        let default = Self {
            size: (WINDOW_WIDTH, WINDOW_HEIGHT),
            tasks: Vec::new(),
        };
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
        let add = Button::default().with_label("@#+");
        header.fixed(&add, HEIGHT);
        add.on_event(move |_| Message::New(description.value()));
        header.end();
        let mut hero = Flex::default().column(); // HERO
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
            let delete = Button::default().with_label("@#1+");
            row.fixed(&delete, HEIGHT);
            delete.on_event(move |_| Message::Delete(idx));
            row.end();
            row.set_frame(FrameType::DownBox);
            hero.fixed(&row, HEIGHT)
        }
        hero.end();
        let footer = Flex::default();
        Frame::default().with_label(&format!("All tasks: {}", self.tasks.len()));
        footer.end(); // FOOTER
        page.end();
        {
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
            Message::Quit => self.quit(),
            Message::Delete(idx) => {
                self.tasks.remove(idx);
            }
            Message::Check(idx) => self.tasks[idx].status = !self.tasks[idx].status,
            Message::New(description) => self.tasks.push(Task {
                status: false,
                description,
            }),
        }
    }
}

impl Model {
    fn quit(&mut self) {
        let file = env::var("HOME").unwrap() + PATH + NAME;
        let window = app::first_window().unwrap();
        self.size = (window.width(), window.height());
        fs::write(file, rmp_serde::to_vec(&self).unwrap()).unwrap();
        app::quit();
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
