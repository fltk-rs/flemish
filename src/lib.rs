/*!
# Flemish

An elmish architecture for fltk-rs.

## Usage
Add flemish to your dependencies:
```toml,ignore
[dependencies]
flemish = "0.3"
```

A usage example:
```rust,no_run
use flemish::{
    button::Button, color_themes, frame::Frame, group::Flex, prelude::*, OnEvent, Sandbox, Settings,
};

pub fn main() {
    Counter::new().run(Settings {
        size: (300, 100),
        resizable: true,
        color_map: Some(color_themes::BLACK_THEME),
        ..Default::default()
    })
}

#[derive(Default)]
struct Counter {
    value: i32,
}

#[derive(Debug, Clone, Copy)]
enum Message {
    IncrementPressed,
    DecrementPressed,
}

impl Sandbox for Counter {
    type Message = Message;

    fn new() -> Self {
        Self::default()
    }

    fn title(&self) -> String {
        String::from("Counter - fltk-rs")
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::IncrementPressed => {
                self.value += 1;
            }
            Message::DecrementPressed => {
                self.value -= 1;
            }
        }
    }

    fn view(&mut self) {
        let col = Flex::default_fill().column();
        Button::default()
            .with_label("Increment")
            .on_event(Message::IncrementPressed);
        Frame::default().with_label(&self.value.to_string());
        Button::default()
            .with_label("Decrement")
            .on_event(Message::DecrementPressed);
        col.end();
    }
}
```
*/
#![allow(clippy::needless_doctest_main)]

use fltk::prelude::*;
pub use fltk::*;
pub use fltk_theme::*;

pub trait OnEvent<W, T>
where
    W: WidgetExt,
    T: Send + Sync + Clone + 'static,
{
    fn on_event(self, msg: T) -> Self
    where
        Self: Sized;
}

impl<W, T> OnEvent<W, T> for W
where
    W: WidgetExt,
    T: Send + Sync + Clone + 'static,
{
    fn on_event(mut self, msg: T) -> Self {
        let (s, _) = app::channel::<T>();
        self.emit(s, msg);
        self
    }
}

#[derive(Default)]
pub struct Settings {
    pub pos: (i32, i32),
    pub size: (i32, i32),
    pub resizable: bool,
    pub background: Option<enums::Color>,
    pub foreground: Option<enums::Color>,
    pub background2: Option<enums::Color>,
    pub inactive: Option<enums::Color>,
    pub selection: Option<enums::Color>,
    pub font: Option<enums::Font>,
    pub font_size: u8,
    pub scheme: Option<app::Scheme>,
    pub color_map: Option<&'static [fltk_theme::ColorMap]>,
    pub theme: Option<fltk_theme::ThemeType>,
}

pub trait Sandbox {
    type Message: Clone + Send + Sync;
    fn new() -> Self;
    fn title(&self) -> String;
    fn view(&mut self);
    fn update(&mut self, message: Self::Message);
    fn run(&mut self, settings: Settings) {
        let a = app::App::default();
        let color_theme = if let Some(color_map) = settings.color_map {
            fltk_theme::ColorTheme::from_colormap(color_map)
        } else {
            fltk_theme::ColorTheme::from_colormap(fltk_theme::color_themes::BLACK_THEME)
        };
        color_theme.apply();
        if let Some(theme) = settings.theme {
            let widget_theme = fltk_theme::WidgetTheme::new(theme);
            widget_theme.apply();
        }
        if let Some(color) = settings.background {
            let c = color.to_rgb();
            app::background(c.0, c.1, c.2);
        }
        if let Some(color) = settings.background2 {
            let c = color.to_rgb();
            app::background2(c.0, c.1, c.2);
        }
        if let Some(color) = settings.foreground {
            let c = color.to_rgb();
            app::foreground(c.0, c.1, c.2);
        }
        if let Some(color) = settings.inactive {
            let c = color.to_rgb();
            app::set_inactive_color(c.0, c.1, c.2);
        }
        if let Some(color) = settings.selection {
            let c = color.to_rgb();
            app::set_inactive_color(c.0, c.1, c.2);
        }
        if settings.font_size != 0 {
            app::set_font_size(settings.font_size);
        }
        if let Some(scheme) = settings.scheme {
            app::set_scheme(scheme);
        } else {
            app::set_scheme(app::Scheme::Gtk);
        }
        if let Some(font) = settings.font {
            app::set_font(font);
        }
        let (w, h) = settings.size;
        let w = if w == 0 { 400 } else { w };
        let h = if h == 0 { 300 } else { h };
        let (x, y) = settings.pos;
        let mut win = window::Window::default()
            .with_size(w, h)
            .with_label(&self.title());
        if (x, y) != (0, 0) {
            win.set_pos(x, y);
        }
        self.view();
        win.end();
        win.make_resizable(settings.resizable);
        win.show();
        let (_, r) = app::channel::<Self::Message>();
        while a.wait() {
            if let Some(msg) = r.recv() {
                self.update(msg);
                win.clear();
                win.begin();
                self.view();
                win.end();
                app::redraw();
            }
        }
    }
}
