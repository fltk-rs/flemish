#![forbid(unsafe_code)]

mod model;

use {
    flemish::{
        app,
        button::Button,
        color_themes,
        enums::{Event, FrameType},
        frame::Frame,
        group::Flex,
        menu::{MenuButton, MenuButtonType},
        prelude::*,
        OnEvent, Sandbox, Settings,
    },
    model::Model,
};

pub fn main() {
    Model::new().run(Settings {
        size: (310, 350),
        color_map: Some(color_themes::DARK_THEME),
        ..Default::default()
    })
}

const NAME: &str = "Sudoku solver";
const PAD: i32 = 10;
const HEIGHT: i32 = PAD * 3;

#[derive(Clone)]
pub enum Message {
    Click((usize, usize, i32)),
    Solve,
    Clear,
}

impl Sandbox for Model {
    type Message = Message;

    fn title(&self) -> String {
        String::from(NAME)
    }

    fn new() -> Self {
        Self::default()
    }

    fn view(&mut self) {
        let mut page = Flex::default_fill().column();
        let mut hero = Flex::default().column();
        for (row, record) in self.grid.iter().enumerate() {
            if row > 0 && row % 3 == 0 {
                hero.fixed(&Frame::default(), PAD);
            };
            let mut flex = Flex::default();
            for (col, cell) in record.iter().enumerate() {
                if col > 0 && col % 3 == 0 {
                    flex.fixed(&Frame::default(), PAD);
                };
                flex.fixed(&crate::frame(row, col, *cell), HEIGHT);
            }
            flex.end();
            flex.set_pad(0);
        }
        hero.end();
        hero.set_pad(0);
        let mut footer = Flex::default();
        {
            Button::default()
                .with_label("Solve")
                .on_event(move |_| Message::Solve);
            Button::default()
                .with_label("Clear")
                .on_event(move |_| Message::Clear);
        }
        footer.end();
        footer.set_pad(PAD);
        page.end();
        page.fixed(&footer, HEIGHT);
        page.set_margin(PAD);
        page.set_pad(PAD);
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::Click((row, col, value)) => self.grid[row][col] = value,
            Message::Solve => self.answer(),
            Message::Clear => self.clear(),
        }
    }
}

fn frame(row: usize, col: usize, value: i32) -> Frame {
    let mut element = Frame::default();
    if value > 0 {
        element.set_label(&value.to_string());
    };
    element.set_frame(FrameType::DownBox);
    element.set_label_size(18);
    element.handle(move |_, event| match event {
        Event::Push => match app::event_mouse_button() {
            app::MouseButton::Right => {
                let mut menu = MenuButton::default().with_type(MenuButtonType::Popup3);
                menu.set_text_size(18);
                menu.add_choice("  1  |  2  |  3  |  4  |  5  |  6  |  7  |  8  |  9  ");
                menu.clone()
                    .on_event(move |choice| Message::Click((row, col, choice.value() + 1)))
                    .popup();
                true
            }
            _ => false,
        },
        _ => false,
    });
    element
}
