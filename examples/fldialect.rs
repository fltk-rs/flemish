#![forbid(unsafe_code)]

use {
    flemish::{
        color_themes,
        app,
        button::{Button,ButtonType}, ColorTheme, frame::Frame, group::Flex, prelude::*, OnEvent, Sandbox, Settings,
        menu::{MenuButton,MenuFlag,Choice},OnMenuEvent,enums::{Shortcut,Color,FrameType,Font},text::{WrapMode,TextBuffer,TextEditor},
        valuator::{Counter,CounterType,Dial},dialog::{FileChooserType,FileChooser,HelpDialog,alert_default},
    },
    std::{env, process::Command, fs, path::Path, thread},
};

pub fn main() {
    if crate::once() {
        let mut app = Model::new();
        app.run(Settings {
            size: (app.width, app.height),
            pos: (app.vertical,app.horizontal),
            ignore_esc_close: true,
            resizable: true,
            color_map: Some(color_themes::DARK_THEME),
            scheme: Some(app::Scheme::Plastic),
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
    theme: bool,
    source: String,
    target: String,
    lang: Vec<String>,
}

#[derive(Clone)]
enum Message {
    Switch(),
    From(u8),
    To(u8),
    Speak(bool),
    Source(String),
    Theme,
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
            theme: params[4] != 0,
            from: params[5],
            to: params[6],
            font: params[7],
            size: params[8],
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

        let mut header = Flex::default(); //HEADER
        crate::menu(&mut header);
        Frame::default();
        let choice = crate::choice("From", &self.lang.join("|"), self.from, &mut header);
        choice.clone().on_event(move |_|Message::From(choice.value() as u8));
        crate::button("Switch", "@#refresh", &mut header).on_event(move |_|Message::Switch);
        let choice = crate::choice("To", &self.lang.join("|"), self.to, &mut header);
        choice.clone().on_event(move |_|Message::To(choice.value() as u8));
        Frame::default();
        let mut button = crate::button("Speak", "@#<", &mut header).with_type(ButtonType::Toggle);
        button.set(self.speak);
        header.fixed(&button, 50);
        button.clone().on_event(move |_|Message::Speak(button.value()));
        header.end();

        let mut hero = Flex::default().column(); //HERO
        let text = crate::text("Source", &self.source, self.theme, self.font, self.size);
        text.clone().on_event(move |_|Message::Source(text.buffer().unwrap().text()));
        crate::text("Target", &self.target, self.theme, self.font, self.size);
        hero.end();

        let mut footer = Flex::default(); //FOOTER
        crate::button("Open...", "@#fileopen", &mut footer).on_event(move |_|Message::Open);
        Frame::default();
        let choice = crate::choice("Font", &app::fonts().join("|"), self.font, &mut footer);
        choice.clone().on_event(move |_|Message::Font(choice.value() as u8));
        crate::button("Translate", "@#circle", &mut footer).on_event(move |_|Message::Translate);
        let counter = crate::counter("Size", self.size as f64, &mut footer).with_type(CounterType::Simple);
        counter.clone().on_event(move |_|Message::Size(counter.value() as u8));
        crate::dial(self.spinner as f64, &mut footer);
        Frame::default();
        crate::button("Save as...", "@#filesaveas", &mut footer).on_event(move |_|Message::Save);
        footer.end();

        page.end();
        {
            header.set_pad(PAD);
            hero.set_pad(PAD);
            footer.set_pad(PAD);
            page.fixed(&header, HEIGHT);
            page.fixed(&footer, HEIGHT);
            page.set_margin(PAD);
            page.set_pad(PAD);
            page.set_frame(FrameType::FlatBox);
            let mut window = page.window().unwrap();
            window.set_label(&format!("Translate from {} to {} - {NAME}", self.lang[self.from as usize], self.lang[self.to as usize]));
            window.size_range(
                DEFAULT[0] as i32 * U8 + DEFAULT[1] as i32,
                DEFAULT[2] as i32 * U8 + DEFAULT[3] as i32,
                0,
                0,
            );
            self.theme();
        }
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::Speak(value) => self.speak = value,
            Message::From(value) => self.from = value,
            Message::To(value) => self.to = value,
            Message::Source(value) => self.source = value,
            Message::Theme => {
                self.theme = !self.theme;
                self.theme();
            }
            Message::Switch() => {
                let temp = self.from;
                self.to = self.from;
                self.from = temp;
            },
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
                self.source = fs::read_to_string(Path::new(&file))
                    .unwrap();
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
                    fs::write(file, self.target.as_bytes())
                    .unwrap();
                };
            };
        } else {
            alert_default("Target is empty.");
        };
    }
    fn translate(&mut self) {
        let from = self.lang[self.from as usize].clone();
        let to = self.lang[self.to as usize].clone();
        let source = self.source.clone();
        if from != to && !source.is_empty() {
            let speak = self.speak;
            let handler = thread::spawn(move || -> String { crate::run(speak, from, to, source) });
            while !handler.is_finished() {
                app::wait();
                app::sleep(0.02);
                if self.spinner == DIAL - 1 {
                    self.spinner = 0;
                } else {
                    self.spinner += 1;
                }
            }
            if let Ok(text) = handler.join() {
                self.target = text;
            };
        };
    }
    fn theme(&mut self) {
        app::set_scheme(if self.theme {
            ColorTheme::new(color_themes::DARK_THEME).apply();
            app::Scheme::Plastic
        } else {
            ColorTheme::new(color_themes::TAN_THEME).apply();
            app::Scheme::Base
        });
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
                self.theme as u8,
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
    flex.fixed(&element, WIDTH - HEIGHT - PAD);
    element
}

fn dial(value: f64, flex: &mut Flex) {
    let mut element = Dial::default();
    element.deactivate();
    element.set_maximum((DIAL / 4 * 3) as f64);
    element.set_value(value);
    flex.fixed(&element, HEIGHT);
}

fn choice(tooltip: &str, choice: &str, value: u8, flex: &mut Flex) -> Choice {
    let mut element = Choice::default();
    element.set_tooltip(tooltip);
    element.add_choice(choice);
    element.set_value(value as i32);
    flex.fixed(&element, WIDTH);
    element
}

fn text(tooltip: &str, value: &str, theme: bool, font: u8, size: u8) -> TextEditor {
    const COLORS: [[Color; 2]; 2] = [
        [Color::from_hex(0xfdf6e3), Color::from_hex(0x586e75)],
        [Color::from_hex(0x002b36), Color::from_hex(0x93a1a1)],
    ];
    let mut element = TextEditor::default();
    element.set_tooltip(tooltip);
    element.set_linenumber_width(HEIGHT);
    element.set_buffer(TextBuffer::default());
    element.wrap_mode(WrapMode::AtBounds, 0);
    element.buffer().unwrap().set_text(value);
    element.set_color(COLORS[theme as usize][0]);
    element.set_text_color(COLORS[theme as usize][1]);
    element.set_text_font(Font::by_index(font as usize));
    element.set_text_size(size as i32);
    element
}

fn menu(flex: &mut Flex) {
    let element = MenuButton::default()
        .with_label("@#menu");
    flex.fixed(&element, 50);
    element.on_item_event(
            "&View/&Night mode\t",
            Shortcut::Ctrl | 'n',
            MenuFlag::Toggle,
            |_| Message::Theme,
        )
        .on_item_event(
            "@#circle  T&ranslate",
            Shortcut::Ctrl | 'r',
            MenuFlag::Normal,
            |_| Message::Translate,
        )
        .on_item_event(
            "@#search  &Info",
            Shortcut::Ctrl | 'i',
            MenuFlag::Normal,
            |_| Message::Info,
        )
        .on_item_event(
            "@#1+  &Quit",
            Shortcut::Ctrl | 'q',
            MenuFlag::Normal,
            |_| Message::Quit,
        );
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
const WIDTH: i32 = HEIGHT * 3;
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
