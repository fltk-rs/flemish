#![forbid(unsafe_code)]

use flemish::{
    app, color_themes,
    group::Flex,
    misc::Progress,
    prelude::*,
    valuator::{Slider, SliderType},
    OnEvent, Sandbox, Settings,
};

pub fn main() {
    Model::new().run(Settings {
        size: (640, 360),
        resizable: false,
        ignore_esc_close: true,
        color_map: Some(color_themes::DARK_THEME),
        scheme: Some(app::Scheme::Base),
        ..Default::default()
    })
}

const PAD: i32 = 10;

#[derive(Clone)]
enum Message {
    SliderChanged(f64),
}

struct Model {
    value: f64,
}

impl Sandbox for Model {
    fn new() -> Self {
        Self { value: 0f64 }
    }

    fn view(&mut self) {
        let mut page = Flex::default_fill()
            .column()
            .with_size(600, 150)
            .center_of_parent();
        {
            crate::progress(self.value);
            crate::slider(self.value)
                .on_event(move |slider| Message::SliderChanged(slider.value()));
        }
        page.end();
        page.set_margin(PAD);
    }

    type Message = Message;
    fn update(&mut self, message: Message) {
        match message {
            Message::SliderChanged(value) => self.value = value,
        }
    }

    fn title(&self) -> String {
        String::from("Progress - Flemish")
    }
}

const MAX: f64 = 100f64;

fn progress(value: f64) {
    let mut element = Progress::default();
    element.set_maximum(MAX);
    element.set_value(value);
}

fn slider(value: f64) -> Slider {
    let mut element = Slider::default().with_type(SliderType::Horizontal);
    element.set_maximum(MAX);
    element.set_precision(0);
    element.set_value(value);
    element
}
