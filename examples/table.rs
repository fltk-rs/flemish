use flemish::{view::*, Settings};

pub fn main() {
    flemish::application("table-app", TableApp::update, TableApp::view)
        .settings(Settings {
            size: (300, 100),
            resizable: true,
            color_map: flemish::theme::color_themes::GRAY_THEME,
            ..Default::default()
        })
        .run();
}

#[derive(Default)]
struct TableApp {}

impl TableApp {
    fn update(&mut self, _message: i32) {}

    fn view(&self) -> View<i32> {
        let v: &[&[&str]] = &[
            &["String1", "String2"],
            &["String3", "String4"],
            &["String5", "String6"],
        ];
        Column::new(&[Table::new(&["Col1", "Col2"], v).view()]).view()
    }
}
