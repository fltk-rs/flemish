#![forbid(unsafe_code)]

use {
    flemish::{
        app,
        button::{Button, ButtonType},
        color_themes,
        dialog::{alert_default, FileChooser, FileChooserType, HelpDialog},
        enums::{Color, Cursor, Event, Font, FrameType, Shortcut},
        frame::Frame,
        group::{Flex, FlexType},
        menu::{Choice, MenuButton, MenuFlag},
        prelude::*,
        text::{TextBuffer, TextEditor, WrapMode},
        valuator::{Counter, CounterType, Dial},
        OnEvent, OnMenuEvent, Sandbox, Settings,
    },
    std::{env, fs, path::Path, process::Command, thread},
};

fn main() {
    if crate::once() {
        let mut app = Model::new();
        app.run(Settings {
            size: (app.width, app.height),
            pos: (app.vertical, app.horizontal),
            ignore_esc_close: true,
            resizable: true,
            color_map: Some(color_themes::DARK_THEME),
            scheme: Some(app::Scheme::Base),
            ..Default::default()
        });
    }
}

struct Model {
    width: i32,
    height: i32,
    vertical: i32,
    horizontal: i32,
    from: u8,
    to: u8,
    speak: bool,
    font: u8,
    size: u8,
    spinner: u8,
    source: String,
    target: String,
    lang: Vec<String>,
}

#[derive(Clone)]
enum Message {
    Switch,
    From(u8),
    To(u8),
    Speak(bool),
    Source(String),
    Size(u8),
    Font(u8),
    Info,
    Translate,
    Open,
    Save,
    Quit,
}

impl Sandbox for Model {
    type Message = Message;

    fn title(&self) -> String {
        String::from(NAME)
    }

    fn new() -> Self {
        let file = env::var("HOME").unwrap() + PATH + NAME;
        let params: Vec<u8> = if Path::new(&file).exists() {
            if let Ok(value) = fs::read(&file) {
                if value.len() == DEFAULT.len() {
                    value
                } else {
                    fs::remove_file(&file).unwrap();
                    Vec::from(DEFAULT)
                }
            } else {
                Vec::from(DEFAULT)
            }
        } else {
            Vec::from(DEFAULT)
        };
        let (w, h) = app::screen_size();
        let width = params[0] as i32 * U8 + params[1] as i32;
        let height = params[2] as i32 * U8 + params[3] as i32;
        let mut lang = crate::list();
        lang.sort();
        Self {
            width,
            height,
            from: params[4],
            to: params[5],
            font: params[6],
            size: params[7],
            vertical: ((w + width as f64) / 4_f64) as i32,
            horizontal: ((h + height as f64) / 4_f64) as i32,
            spinner: 0,
            speak: false,
            source: String::new(),
            target: String::new(),
            lang,
        }
    }

    fn view(&mut self) {
        let mut page = Flex::default_fill().column();

        let mut header = Flex::default();
        header.fixed(&crate::menu(), HEIGHT);
        Frame::default();
        crate::choice("From", &self.lang.join("|"), self.from, &mut header)
            .on_event(move |choice| Message::From(choice.value() as u8));
        crate::button("Switch", "@#refresh", &mut header)
            .on_event(move |_| Message::Switch);
        crate::choice("To", &self.lang.join("|"), self.to, &mut header)
            .on_event(move |choice| Message::To(choice.value() as u8));
        Frame::default();
        let mut button = crate::button("Speak", "@#<", &mut header).with_type(ButtonType::Toggle);
        button.set(self.speak);
        button.on_event(move |button| Message::Speak(button.value()));
        header.end();

        let mut hero = Flex::default().column().with_id("HERO");
        crate::text("Source", &self.source, self.font, self.size)
            .on_event(move |text| Message::Source(text.buffer().unwrap().text()));
        Frame::default().with_id("Handle").handle(move |frame, event| {
            let mut flex = app::widget_from_id::<Flex>("HERO").unwrap();
            match event {
                Event::Push => true,
                Event::Drag => {
                    let child = flex.child(0).unwrap();
                    match flex.get_type() {
                        FlexType::Column => {
                            if (flex.y()..=flex.height() + flex.y() - frame.height())
                                .contains(&app::event_y())
                            {
                                flex.fixed(&child, app::event_y() - flex.y());
                            }
                        }
                        FlexType::Row => {
                            if (flex.x()..=flex.width() + flex.x() - frame.width())
                                .contains(&app::event_x())
                            {
                                flex.fixed(&child, app::event_x() - flex.x());
                            }
                        }
                    }
                    app::redraw();
                    true
                }
                Event::Enter => {
                    frame.window().unwrap().set_cursor(
                        match flex.get_type() {
                            FlexType::Column => Cursor::NS,
                            FlexType::Row => Cursor::WE,
                        }
                    );
                    true
                }
                Event::Leave => {
                    frame.window().unwrap().set_cursor(Cursor::Arrow);
                    true
                }
                _ => false,
            }
        });
        crate::text("Target", &self.target, self.font, self.size);
        hero.end();

        let mut footer = Flex::default(); //FOOTER
        crate::button("Open...", "@#fileopen", &mut footer).on_event(move |_| Message::Open);
        Frame::default();
        crate::choice("Font", &app::fonts().join("|"), self.font, &mut footer)
            .on_event(move |choice| Message::Font(choice.value() as u8));
        crate::button("Translate", "@#circle", &mut footer).on_event(move |_| Message::Translate);
        crate::counter("Size", self.size as f64, &mut footer).with_type(CounterType::Simple)
            .on_event(move |counter| Message::Size(counter.value() as u8));
        footer.fixed(&crate::dial(self.spinner as f64), HEIGHT);
        Frame::default();
        crate::button("Save as...", "@#filesaveas", &mut footer).on_event(move |_| Message::Save);
        footer.end();

        page.end();
        {
            header.set_pad(0);
            hero.set_pad(0);
            hero.fixed(&hero.child(1).unwrap(), PAD);
            hero.handle(move |flex, event| {
                if event == Event::Resize {
                    flex.set_type(
                        match flex.width() < flex.height() {
                            true => FlexType::Column,
                            false => FlexType::Row,
                        }
                    );
                    flex.fixed(&flex.child(0).unwrap(), 0);
                    flex.fixed(&flex.child(1).unwrap(), PAD);
                    true
                } else {
                    false
                }
            });
            footer.set_pad(0);
            page.fixed(&header, HEIGHT);
            page.fixed(&footer, HEIGHT);
            page.set_margin(PAD);
            page.set_pad(PAD);
            page.set_frame(FrameType::FlatBox);
            let mut window = page.window().unwrap();
            window.set_xclass(NAME);
            window.set_label(&format!(
                "Translate from {} to {} - {NAME}",
                self.lang[self.from as usize], self.lang[self.to as usize]
            ));
            window.size_range(
                DEFAULT[0] as i32 * U8 + DEFAULT[1] as i32,
                DEFAULT[0] as i32 * U8 + DEFAULT[1] as i32,
                0,
                0,
            );
        }
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::Speak(value) => self.speak = value,
            Message::From(value) => self.from = value,
            Message::To(value) => self.to = value,
            Message::Source(value) => self.source = value,
            Message::Switch => {
                let temp = self.from;
                self.from = self.to;
                self.to = temp;
            }
            Message::Font(value) => self.font = value,
            Message::Size(value) => self.size = value,
            Message::Open => self.open(),
            Message::Save => self.save(),
            Message::Translate => self.translate(),
            Message::Info => crate::info(),
            Message::Quit => self.quit(),
        }
    }
}

impl Model {
    fn open(&mut self) {
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
    fn save(&self) {
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
    fn translate(&mut self) {
        let mut button = app::widget_from_id::<Button>("Translate").unwrap();
        button.deactivate();
        let from = self.lang[self.from as usize].clone();
        let to = self.lang[self.to as usize].clone();
        let source = self.source.clone();
        if from != to && !source.is_empty() {
            let speak = self.speak;
            let handler = thread::spawn(move || -> String { crate::run(speak, from, to, source) });
            while !handler.is_finished() {
                app::wait();
                app::sleep(0.02);
                app::widget_from_id::<Dial>("SPINNER")
                    .unwrap()
                    .do_callback();
            }
            if let Ok(text) = handler.join() {
                self.target = text;
            };
        };
        button.activate();
    }
    fn quit(&self) {
        let file = env::var("HOME").unwrap() + PATH + NAME;
        let window = app::first_window().unwrap();
        fs::write(
            file,
            [
                (window.width() / U8) as u8,
                (window.width() % U8) as u8,
                (window.height() / U8) as u8,
                (window.height() % U8) as u8,
                self.from,
                self.to,
                self.font,
                self.size,
            ],
        )
        .unwrap();
        app::quit();
    }
}

fn button(tooltip: &str, label: &str, flex: &mut Flex) -> Button {
    let mut element = Button::default().with_label(label).with_id(tooltip);
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

fn dial(value: f64) -> Dial {
    let mut element = Dial::default().with_id("SPINNER");
    element.deactivate();
    element.set_maximum((DIAL / 4 * 3) as f64);
    element.set_value(value);
    element.set_callback(move |dial| {
        dial.set_value(if dial.value() == (DIAL - 1) as f64 {
            dial.minimum()
        } else {
            dial.value() + 1f64
        })
    });
    element
}

fn choice(tooltip: &str, choice: &str, value: u8, flex: &mut Flex) -> Choice {
    let mut element = Choice::default();
    element.set_tooltip(tooltip);
    element.add_choice(choice);
    element.set_value(value as i32);
    flex.fixed(&element, WIDTH);
    element
}

fn text(tooltip: &str, value: &str, font: u8, size: u8) -> TextEditor {
    let mut element = TextEditor::default().with_id(tooltip);
    element.set_tooltip(tooltip);
    element.set_linenumber_width(HEIGHT);
    element.set_buffer(TextBuffer::default());
    element.wrap_mode(WrapMode::AtBounds, 0);
    element.buffer().unwrap().set_text(value);
    element.set_color(Color::from_hex(0x002b36));
    element.set_text_color(Color::from_hex(0x93a1a1));
    element.set_text_font(Font::by_index(font as usize));
    element.set_text_size(size as i32);
    element
}

fn menu() -> MenuButton {
    let element = MenuButton::default();
    element.clone()
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
    element
}

fn info() {
    const INFO: &str = r#"<p>
<a href="https://github.com/fltk-rs/demos/blob/master/fldialect">FlDialect</a>
 is similar to
 <a href="https://apps.gnome.org/Dialect">Dialect</a>
 written using
 <a href="https://fltk-rs.github.io/fltk-rs">FLTK-RS</a>
</p>"#;
    let mut dialog = HelpDialog::default();
    dialog.set_value(INFO);
    dialog.set_text_size(16);
    dialog.show();
    while dialog.shown() {
        app::wait();
    }
}

pub fn run(voice: bool, from: String, to: String, word: String) -> String {
    let run = Command::new("trans")
        .args([
            "-join-sentence",
            "-no-ansi",
            "-show-languages",
            "n",
            "-show-original",
            "n",
            "-show-original-dictionary",
            "n",
            "-show-original-dictionary",
            "n",
            "-show-prompt-message",
            "n",
            "-show-alternatives",
            "n",
            "-show-translation-phonetics",
            "n",
            "-indent",
            "2",
            "-source",
            &from,
            "-target",
            &to,
            match word.split_whitespace().count() {
                1 => "",
                _ => "-brief",
            },
            if voice { "-speak" } else { "" },
            &word.trim().replace("\n\n", "\n"),
        ])
        .output()
        .expect("failed to execute bash");
    String::from_utf8_lossy(match run.status.success() {
        true => &run.stdout,
        false => &run.stderr,
    })
    .to_string()
}

pub fn list() -> Vec<String> {
    if cfg!(target_family = "unix") {
        let run = Command::new("trans")
            .arg("-list-languages-english")
            .output()
            .expect("failed to execute bash");
        match run.status.success() {
            true => String::from_utf8_lossy(&run.stdout)
                .lines()
                .map(str::to_string)
                .collect::<Vec<String>>(),
            false => panic!("\x1b[31m{}\x1b[0m", String::from_utf8_lossy(&run.stderr)),
        }
    } else {
        Vec::from([String::from("no way")])
    }
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

const NAME: &str = "FlDialect";
const PATH: &str = "/.config";
const DIAL: u8 = 120;
const PAD: i32 = 10;
const HEIGHT: i32 = PAD * 3;
const WIDTH: i32 =  125;
const U8: i32 = 255;
const DEFAULT: [u8; 9] = [
    1,   // [0] window_width * U8 +
    105, // [1] window_width_fract
    2,   // [2] window_height * U8 +
    130, // [3] window_height_fract
    0,   // [4] theme
    119, // [5] header_from
    35,  // [6] header_to
    1,   // [7] footer_font
    14,  // [8] footer_size
];
