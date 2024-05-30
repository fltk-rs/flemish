use flemish::{
    app, color_themes, frame::Frame, enums::{FrameType,CallbackTrigger}, group::Flex, input::{Input,InputType}, prelude::*,
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

#[derive(Debug, Clone)]
enum Message {
    Celsius(i32),
    Fahrenheit(i32),
}

const WIDTH: i32 = 90;

struct Model {
    celsius: i32,
    fahrenheit: i32,
}

impl Model {
    fn celsius(&mut self, value: i32) {
        self.fahrenheit = value;
        self.celsius = ((value as f64 - 32f64) * (5f64 / 9f64)).round() as i32
    }
    fn fahrenheit(&mut self, value: i32) {
        self.celsius = value;
        self.fahrenheit = (value as f64 * (9f64 / 5f64) + 32f64).round() as i32
    }
}

impl Sandbox for Model {
    type Message = Message;

    fn new() -> Self {
        Self {
            celsius: 0,
            fahrenheit: 32,
        }
    }

    fn title(&self) -> String {
        format!("{} : {} - 7GUI: Temperature", self.celsius, self.fahrenheit)
    }

    fn view(&mut self) {
        let mut page = Flex::default().with_size(WIDTH * 3, WIDTH).center_of_parent();
        {
            Frame::default();
            let mut right = Flex::default().column();
            crate::input(self.celsius)
                .with_label("Celsius: ")
                .on_event(move |input| Message::Celsius(input.value().parse::<i32>().unwrap()));
            crate::input(self.fahrenheit)
                .with_label("Fahrenheit: ")
                .on_event(move |input| Message::Fahrenheit(input.value().parse::<i32>().unwrap()));
            right.end();
            right.set_pad(10);
        }
        page.end();
        page.set_pad(0);
        page.set_margin(10);
        page.set_frame(FrameType::UpBox);
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::Celsius(value) => self.fahrenheit(value),
            Message::Fahrenheit(value) => self.celsius(value),
        }
    }
}

fn input(value: i32) -> Input {
    let mut element = Input::default().with_type(InputType::Int);
    element.set_value(&value.to_string());
    element
}
