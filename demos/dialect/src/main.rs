#![forbid(unsafe_code)]

mod model;

use {
    flemish::{
        app,
        button::Button,
        color_themes,
        dialog::{alert_default, FileChooser, FileChooserType, HelpDialog},
        enums::{Color, Event, Font, FrameType, Shortcut},
        frame::Frame,
        group::Flex,
        menu::{Choice, MenuButton, MenuFlag},
        prelude::*,
        text::{TextBuffer, TextDisplay, TextEditor, WrapMode},
        valuator::{Counter, CounterType},
        OnEvent, OnMenuEvent, Sandbox, Settings,
    },
    model::Model,
    std::{env, fs, path::Path, process::Command, thread},
};

fn main() {
    if crate::once() {
        app::GlobalState::<String>::new(env::var("HOME").unwrap() + "/.config" + NAME);
        Model::new().run(Settings {
            ignore_esc_close: true,
            resizable: false,
            size: (360, 640),
            color_map: Some(color_themes::DARK_THEME),
            scheme: Some(app::Scheme::Base),
            ..Default::default()
        });
    }
}

#[derive(Clone)]
pub enum Message {
    Switch,
    From(i32),
    To(i32),
    Source(String),
    Size(i32),
    Font(i32),
    Translate,
    Info,
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
        let mut page = Flex::default_fill().column();
        {
            let mut header = Flex::default();
            crate::menu(&mut header);
            Frame::default();
            let lang = self
                .lang
                .iter()
                .map(|x| x["name"].clone())
                .collect::<Vec<String>>()
                .join("|");
            crate::choice("From", &lang, self.from, &mut header)
                .on_event(move |choice| Message::From(choice.value()));
            crate::button("Switch", "@#refresh", &mut header).on_event(move |_| Message::Switch);
            crate::choice("To", &lang, self.to, &mut header)
                .on_event(move |choice| Message::To(choice.value()));
            Frame::default();
            crate::button("Translate", "@#circle", &mut header)
                .on_event(move |_| Message::Translate);
            header.end();
            header.set_pad(0);
            page.fixed(&header, HEIGHT);
        }
        {
            let mut hero = Flex::default_fill().column();
            crate::texteditor("Source", &self.source, self.font, self.size)
                .on_event(move |text| Message::Source(text.buffer().unwrap().text()));
            hero.fixed(&Frame::default(), PAD);
            crate::textdisplay("Target", &self.target, self.font, self.size);
            hero.end();
            hero.set_pad(0);
        }
        {
            let mut footer = Flex::default(); //FOOTER
            crate::choice("Font", &app::fonts().join("|"), self.font, &mut footer)
                .on_event(move |choice| Message::Font(choice.value()));
            Frame::default();
            crate::counter("Size", self.size as f64, &mut footer)
                .with_type(CounterType::Simple)
                .on_event(move |counter| Message::Size(counter.value() as i32));
            footer.end();
            footer.set_pad(0);
            page.fixed(&footer, HEIGHT);
        }
        page.end();
        page.set_margin(PAD);
        page.set_pad(PAD);
        page.set_frame(FrameType::FlatBox);
    }

    fn update(&mut self, message: Message) {
        match message {
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
            Message::Info => crate::info(),
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
                        self.source = fs::read_to_string(Path::new(&file)).unwrap();
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
                            fs::write(file, self.target.as_bytes()).unwrap();
                        };
                    };
                } else {
                    alert_default("Target is empty.");
                };
            }
            Message::Translate => {
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

fn counter(tooltip: &str, value: f64, flex: &mut Flex) -> Counter {
    let mut element = Counter::default();
    element.set_tooltip(tooltip);
    element.set_range(14_f64, 22_f64);
    element.set_precision(0);
    element.set_value(value);
    flex.fixed(&element, WIDTH - HEIGHT);
    element
}

fn info() {
    const INFO: &str = r#"<p style="color:gray;">
<a href="https://github.com/fltk-rs/demos/tree/master/fldialect">FlDialect</a>
 is similar to
 <a href="https://apps.gnome.org/Dialect">Dialect</a>
 written using
 <a href="https://fltk-rs.github.io/fltk-rs">FLTK-RS</a>
</p>"#;
    let (r, g, b) = Color::from_hex(0x2aa198).to_rgb();
    app::set_color(Color::Blue, r, g, b);
    let mut dialog = HelpDialog::default();
    dialog.set_value(INFO);
    dialog.set_text_size(16);
    dialog.show();
    while dialog.shown() {
        app::wait();
    }
}

fn choice(tooltip: &str, choice: &str, value: i32, flex: &mut Flex) -> Choice {
    let mut element = Choice::default();
    element.set_tooltip(tooltip);
    element.add_choice(choice);
    element.set_value(value);
    flex.fixed(&element, WIDTH);
    element
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
            move |_| Message::Translate,
        )
        .on_item_event(
            "@#search  &Info",
            Shortcut::Ctrl | 'i',
            MenuFlag::Normal,
            move |_| Message::Info,
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

const SPINNER: Event = Event::from_i32(405);
const NAME: &str = "FlDialect";
const PAD: i32 = 10;
const HEIGHT: i32 = PAD * 3;
const WIDTH: i32 = 125;
