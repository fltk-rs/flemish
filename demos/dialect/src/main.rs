#![forbid(unsafe_code)]

mod model;

use {
    flemish::{
        app,
        button::Button,
        color_themes,
        dialog::{alert_default, FileChooser, FileChooserType},
        enums::{Align, Color, Event, Font, FrameType, Shortcut},
        frame::Frame,
        group::{Flex, FlexType, Wizard},
        menu::{Choice, MenuButton, MenuButtonType, MenuFlag},
        misc::HelpView,
        prelude::*,
        text::{TextBuffer, TextDisplay, TextEditor, WrapMode},
        valuator::{Counter, CounterType},
        OnEvent, OnMenuEvent, Sandbox, Settings,
    },
    model::Model,
    std::{env, process::Command, thread},
};

const SPINNER: Event = Event::from_i32(405);
const NAME: &str = "FlDialect";
const PAD: i32 = 10;
const HEIGHT: i32 = PAD * 3;
const WIDTH: i32 = 105;

fn main() {
    if crate::once() {
        app::GlobalState::<String>::new(env::var("HOME").unwrap() + "/.config/" + NAME);
        Model::new().run(Settings {
            ignore_esc_close: true,
            resizable: true,
            size: (360, 640),
            size_range: Some((360, 640, 0, 0)),
            color_map: Some(color_themes::DARK_THEME),
            scheme: Some(app::Scheme::Base),
            ..Default::default()
        });
    }
}

#[derive(Clone)]
pub enum Message {
    From(i32),
    To(i32),
    Size(i32),
    Font(i32),
    Page(i32),
    Source(String),
    Switch,
    Click,
    Open,
    Save,
    Quit,
}

impl Sandbox for Model {
    type Message = Message;

    fn title(&self) -> String {
        format!(
            "Translate from {} to {} - {NAME}",
            self.lang[self.from as usize]["name"], self.lang[self.to as usize]["name"]
        )
    }

    fn new() -> Self {
        let file = app::GlobalState::<String>::get().with(move |file| file.clone());
        Model::default(&file)
    }

    fn view(&mut self) {
        let mut wizard = Wizard::default_fill();
        {
            let mut page = Flex::default_fill().column();
            {
                let mut header = Flex::default();
                {
                    crate::menu(&mut header);
                    Frame::default();
                    let lang = self
                        .lang
                        .iter()
                        .map(|x| x["name"].clone())
                        .collect::<Vec<String>>()
                        .join("|");
                    header.fixed(
                        &crate::choice("From", &lang, self.from)
                            .clone()
                            .on_event(move |choice| Message::From(choice.value())),
                        WIDTH,
                    );
                    crate::button("Switch", "@#refresh", &mut header)
                        .on_event(move |_| Message::Switch);
                    header.fixed(
                        &crate::choice("To", &lang, self.to)
                            .clone()
                            .on_event(move |choice| Message::To(choice.value())),
                        WIDTH,
                    );
                    Frame::default();
                    crate::button("Translate", "@#circle", &mut header)
                        .on_event(move |_| Message::Click);
                }
                header.end();
                header.set_pad(PAD);
                page.fixed(&header, HEIGHT);
                let mut hero = Flex::default_fill();
                {
                    crate::texteditor("Source", &self.source, self.font, self.size)
                        .on_event(move |text| Message::Source(text.buffer().unwrap().text()));
                    Frame::default();
                    crate::textdisplay("Target", &self.target, self.font, self.size);
                }
                hero.end();
                hero.set_pad(0);
                crate::orientation(&mut hero);
                hero.handle(crate::resize);
            }
            page.end();
            page.set_pad(PAD);
            page.set_margin(PAD);
            page.set_frame(FrameType::FlatBox);
            let mut page = Flex::default_fill();
            {
                crate::info();
            }
            page.end();
            page.set_margin(PAD);
            page.handle(crate::back);
            let mut page = Flex::default_fill();
            {
                Frame::default();
                let mut right = Flex::default_fill().column();
                {
                    right.fixed(
                        &crate::choice("Font", &app::fonts().join("|"), self.font)
                            .with_label("Font")
                            .clone()
                            .on_event(move |choice| Message::Font(choice.value())),
                        HEIGHT,
                    );
                    right.fixed(
                        &crate::counter("Size", self.size as f64)
                            .with_label("Size")
                            .clone()
                            .on_event(move |counter| Message::Size(counter.value() as i32)),
                        HEIGHT,
                    );
                }
                right.end();
                right.set_pad(PAD);
            }
            page.end();
            page.set_margin(PAD);
            page.handle(crate::back);
        }
        wizard.end();
        wizard.set_current_widget(&wizard.child(self.page).unwrap());
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::Page(value) => self.page = value,
            Message::Quit => {
                let file = app::GlobalState::<String>::get().with(move |file| file.clone());
                self.save(&file);
                app::quit();
            }
            Message::From(value) => self.from = value,
            Message::To(value) => self.to = value,
            Message::Source(value) => self.source = value,
            Message::Switch => std::mem::swap(&mut self.from, &mut self.to),
            Message::Font(value) => self.font = value,
            Message::Size(value) => self.size = value,
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
                        self.open(&file);
                    };
                };
            }
            Message::Save => {
                if !self.target.is_empty() {
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
                            self.target(&file)
                        };
                    };
                } else {
                    alert_default("Target is empty.");
                };
            }
            Message::Click => {
                let clone = self.clone();
                if clone.from != clone.to && !clone.source.is_empty() {
                    let handler = thread::spawn(move || -> String { clone.click() });
                    while !handler.is_finished() {
                        app::wait();
                        app::handle_main(SPINNER).unwrap();
                        app::sleep(0.02);
                    }
                    if let Ok(text) = handler.join() {
                        self.target = text;
                    };
                };
            }
        }
    }
}

fn button(tooltip: &str, label: &str, flex: &mut Flex) -> Button {
    let mut element = Button::default().with_label(label);
    element.set_tooltip(tooltip);
    element.set_label_size(HEIGHT / 2);
    flex.fixed(&element, HEIGHT);
    element
}

fn counter(tooltip: &str, value: f64) -> Counter {
    let mut element = Counter::default()
        .with_type(CounterType::Simple)
        .with_align(Align::Left);
    element.set_tooltip(tooltip);
    element.set_range(14_f64, 22_f64);
    element.set_precision(0);
    element.set_value(value);
    element
}

fn info() {
    let (r, g, b) = Color::from_hex(0x2aa198).to_rgb();
    app::set_color(Color::Blue, r, g, b);
    let mut help = HelpView::default();
    help.set_value(include_str!("../README.md"));
    help.set_text_size(16);
}

fn back(_: &mut Flex, event: Event) -> bool {
    match event {
        Event::Push => match app::event_mouse_button() {
            app::MouseButton::Right => {
                MenuButton::default()
                    .with_type(MenuButtonType::Popup3)
                    .clone()
                    .on_item_event("@<-", Shortcut::None, MenuFlag::Normal, move |_| {
                        Message::Page(0)
                    })
                    .popup();
                true
            }
            _ => false,
        },
        _ => false,
    }
}

fn choice(tooltip: &str, choice: &str, value: i32) -> Choice {
    let mut element = Choice::default();
    element.set_tooltip(tooltip);
    element.add_choice(choice);
    element.set_value(value);
    element
}

fn resize(flex: &mut Flex, event: Event) -> bool {
    if event == Event::Resize {
        crate::orientation(flex);
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
    flex.fixed(&flex.child(1).unwrap(), PAD);
}

fn texteditor(tooltip: &str, value: &str, font: i32, size: i32) -> TextEditor {
    let mut element = TextEditor::default();
    element.set_tooltip(tooltip);
    element.set_linenumber_width(HEIGHT);
    element.set_buffer(TextBuffer::default());
    element.wrap_mode(WrapMode::AtBounds, 0);
    element.buffer().unwrap().set_text(value);
    element.set_color(Color::from_hex(0x002b36));
    element.set_text_color(Color::from_hex(0x93a1a1));
    element.set_text_font(Font::by_index(font as usize));
    element.set_text_size(size);
    element.set_linenumber_size(size);
    element
}

fn textdisplay(tooltip: &str, value: &str, font: i32, size: i32) {
    let mut element = TextDisplay::default();
    element.set_tooltip(tooltip);
    element.set_linenumber_width(HEIGHT);
    element.set_buffer(TextBuffer::default());
    element.wrap_mode(WrapMode::AtBounds, 0);
    element.buffer().unwrap().set_text(value);
    element.set_color(Color::from_hex(0x002b36));
    element.set_text_color(Color::from_hex(0x93a1a1));
    element.set_text_font(Font::by_index(font as usize));
    element.set_text_size(size);
    element.set_linenumber_size(size);
    element.handle(move |display, event| {
        if event == crate::SPINNER {
            display.insert("#");
            true
        } else {
            false
        }
    });
}

fn menu(flex: &mut Flex) {
    let element = MenuButton::default();
    element
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
        .on_item_event(
            "@#circle  T&ranslate",
            Shortcut::Ctrl | 'r',
            MenuFlag::Normal,
            move |_| Message::Click,
        )
        .on_item_event(
            "@#search  &Info",
            Shortcut::Ctrl | 'i',
            MenuFlag::Normal,
            move |_| Message::Page(1),
        )
        .on_item_event(
            "@#menu  Se&ttings",
            Shortcut::Ctrl | 't',
            MenuFlag::Normal,
            move |_| Message::Page(2),
        )
        .on_item_event(
            "@#1+  &Quit",
            Shortcut::Ctrl | 'q',
            MenuFlag::Normal,
            move |_| Message::Quit,
        );
    flex.fixed(&element, HEIGHT);
}

pub fn once() -> bool {
    if cfg!(target_os = "linux") {
        let run = Command::new("lsof")
            .args(["-t", env::current_exe().unwrap().to_str().unwrap()])
            .output()
            .expect("failed to execute bash");
        match run.status.success() {
            true => {
                String::from_utf8_lossy(&run.stdout)
                    .split_whitespace()
                    .count()
                    == 1
            }
            false => panic!("\x1b[31m{}\x1b[0m", String::from_utf8_lossy(&run.stderr)),
        }
    } else {
        true
    }
}
