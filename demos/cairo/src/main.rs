mod model;

use {
    cairo::Context,
    flemish::{
        app,
        frame::Frame,
        color_themes,
        enums::Color,
        prelude::*,
        OnEvent, Sandbox, Settings,
    },
    model::Model,
};

#[derive(Clone, Copy)]
pub enum Message {
    Change(usize),
}

fn main() {
    Model::new().run(Settings {
        size: (260, 260),
        ignore_esc_close: true,
        resizable: false,
        background: Some(Color::White),
        color_map: Some(color_themes::TAN_THEME),
        scheme: Some(app::Scheme::Base),
        ..Default::default()
    })
}

impl Sandbox for Model {
    type Message = Message;

    fn new() -> Self {
        Self::default()
    }

    fn title(&self) -> String {
        format!("{} - FlCairo", self.state[0])
    }

    fn view(&mut self) {
        fltk::app::cairo::set_autolink_context(true);
        let mut frame = cairowidget(5, 5, 100, 100, "Box1");
        frame.set_color(match self.state[0] {
            true => Color::Red,
            false => Color::DarkRed,
        });
        frame.on_event(move |_| Message::Change(0));
        let mut frame = cairowidget(80, 80, 100, 100, "Box2");
        frame.set_color(match self.state[1] {
            true => Color::Yellow,
            false => Color::DarkYellow,
        });
        frame.on_event(move |_| Message::Change(1));
        let mut frame = cairowidget(155, 155, 100, 100, "Box3");
        frame.set_color(match self.state[2] {
            true => Color::Green,
            false => Color::DarkGreen,
        });
        frame.clone().on_event(move |_| Message::Change(2));
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::Change(idx) => self.change(idx),
        }
    }
}

fn draw_box_with_alpha(rect: &mut Frame) {
    let ctx = unsafe { Context::from_raw_none(fltk::app::cairo::cc() as _) };
    let (r, g, b) = rect.color().to_rgb();
    ctx.save().unwrap();
    ctx.move_to(rect.x() as f64, rect.y() as f64);
    ctx.line_to((rect.x() + rect.w()) as f64, rect.y() as f64);
    ctx.line_to((rect.x() + rect.w()) as f64, (rect.y() + rect.h()) as f64);
    ctx.line_to(rect.x() as f64, (rect.y() + rect.h()) as f64);
    ctx.close_path();
    ctx.set_source_rgba(
        r as f64 / 255.0,
        g as f64 / 255.0,
        b as f64 / 255.0,
        100.0 / 255.0,
    );
    ctx.fill().unwrap();
    ctx.restore().unwrap();
}

pub fn cairowidget(x: i32, y: i32, w: i32, h: i32, label: &str) -> Frame {
    let mut element = Frame::new(x, y, w, h, None).with_label(label);
    element.super_draw_first(false); // required for windows
    element.draw(draw_box_with_alpha);
    element
}
