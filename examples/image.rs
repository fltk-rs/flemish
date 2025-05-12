use flemish::{enums::*, view::*, Image, Settings};

pub fn main() {
    flemish::application("image", State::update, State::view)
        .settings(Settings {
            size: (300, 100),
            resizable: true,
            ..Default::default()
        })
        .run();
}

#[derive(Default)]
struct State {}

impl State {
    fn update(&mut self, _message: ()) {}

    fn view(&self) -> View<()> {
        Column::new(&[Frame::new("Enter name:")
            .image(Some(
                Image::load("../fltk-rs/screenshots/calc.jpg").unwrap(),
            ))
            .align(Align::TextOverImage)
            .view()])
        .view()
    }
}
