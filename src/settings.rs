use fltk::*;

#[derive(Clone, Debug)]
pub struct Settings<Message> {
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
    pub menu_linespacing: Option<i32>,
    pub ignore_esc_close: bool,
    pub size_range: Option<(i32, i32, i32, i32)>,
    pub on_close: Option<Message>,
    pub worker_threads: Option<usize>,
}

impl<Message> Default for Settings<Message> {
    fn default() -> Self {
        Self {
            pos: (0, 0),
            size: (0, 0),
            resizable: true,
            background: None,
            foreground: None,
            background2: None,
            inactive: None,
            selection: None,
            font: None,
            font_size: 0,
            scheme: None,
            color_map: None,
            theme: None,
            menu_linespacing: None,
            ignore_esc_close: false,
            size_range: None,
            on_close: None,
            worker_threads: None,
        }
    }
}
