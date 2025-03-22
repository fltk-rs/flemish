use fltk::*;

#[derive(Clone, Debug)]
pub struct Settings<Message> {
    pub pos: (i32, i32),
    pub size: (i32, i32),
    pub resizable: bool,
    pub font_size: u8,
    pub scheme: app::Scheme,
    pub color_map: &'static [fltk_theme::ColorMap],
    pub menu_linespacing: i32,
    pub ignore_esc_close: bool,
    pub background: Option<enums::Color>,
    pub foreground: Option<enums::Color>,
    pub background2: Option<enums::Color>,
    pub inactive: Option<enums::Color>,
    pub selection: Option<enums::Color>,
    pub font: Option<enums::Font>,
    pub theme: Option<fltk_theme::ThemeType>,
    pub size_range: Option<(i32, i32, i32, i32)>,
    pub on_close: Option<Message>,
    pub worker_threads: Option<usize>,
}

impl<Message> Default for Settings<Message> {
    fn default() -> Self {
        Self {
            pos: (0, 0),
            size: (600, 400),
            resizable: true,
            font_size: 14,
            scheme: app::Scheme::Oxy,
            color_map: fltk_theme::color_themes::BLACK_THEME,
            menu_linespacing: 10,
            ignore_esc_close: false,
            background: None,
            foreground: None,
            background2: None,
            inactive: None,
            selection: None,
            font: None,
            theme: None,
            size_range: None,
            on_close: None,
            worker_threads: None,
        }
    }
}
