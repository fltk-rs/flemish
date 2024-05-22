#![forbid(unsafe_code)]

use {
    cairo::{Context, Format, ImageSurface},
    flemish::{
        app,
        button::Button,
        color_themes, draw,
        enums::{Align, Color, ColorDepth, Font, Shortcut},
        frame::Frame,
        group::Flex,
        image::RgbImage,
        menu::{MenuButton, MenuFlag},
        prelude::*,
        OnEvent, OnMenuEvent, Sandbox, Settings,
    },
};

const PAD: i32 = 10;
const HEIGHT: i32 = 3 * PAD;
const WIDTH: i32 = 3 * HEIGHT;
const NAME: &str = "FlCairo";

#[derive(Clone, Copy)]
enum Message {
    Inc,
    Dec,
    Quit,
}

pub fn main() {
    Counter::new().run(Settings {
        size: (640, 360),
        ignore_esc_close: true,
        background: Some(Color::White),
        color_map: Some(color_themes::TAN_THEME),
        scheme: Some(app::Scheme::Plastic),
        ..Default::default()
    })
}

struct Counter {
    value: u8,
}

impl Sandbox for Counter {
    type Message = Message;

    fn title(&self) -> String {
        format!("{} - {}", self.value, crate::NAME)
    }

    fn new() -> Self {
        Self { value: 0u8 }
    }

    fn view(&mut self) {
        let mut page = Flex::default_fill().column();

        let mut header = Flex::default(); //HEADER
        header.fixed(&crate::menu(), WIDTH);
        header.end();

        let hero = Flex::default(); //HERO
        crate::cairobutton()
            .with_label("@#<")
            .on_event(move |_| Message::Dec);
        crate::frame(&self.value.to_string());
        crate::cairobutton()
            .with_label("@#>")
            .on_event(move |_| Message::Inc);
        hero.end();

        page.end();
        page.set_pad(PAD);
        page.set_margin(PAD);
        page.fixed(&header, HEIGHT);
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::Inc => {
                self.value = self.value.saturating_add(1);
            }
            Message::Dec => {
                self.value = self.value.saturating_sub(1);
            }
            Message::Quit => {
                app::quit();
            }
        }
    }
}

fn menu() -> MenuButton {
    MenuButton::default()
        .with_label("@#menu")
        .on_item_event(
            "Command/Increment",
            Shortcut::None,
            MenuFlag::Normal,
            move |_| Message::Inc,
        )
        .on_item_event(
            "Command/Decrement",
            Shortcut::None,
            MenuFlag::Normal,
            move |_| Message::Dec,
        )
        .on_item_event("Quit", Shortcut::Ctrl | 'q', MenuFlag::Normal, move |_| {
            Message::Quit
        })
}

fn frame(value: &str) {
    let mut element = Frame::default().with_label(value);
    element.set_label_size(60);
}

fn cairobutton() -> Button {
    let mut element = Button::default();
    element.super_draw(false);
    element.draw(|w| {
        draw::draw_rect_fill(w.x(), w.y(), w.w(), w.h(), Color::White);
        let mut surface =
            ImageSurface::create(Format::ARgb32, w.w(), w.h()).expect("Couldn’t create surface");
        crate::draw_surface(&mut surface, w.w(), w.h());
        if !w.value() {
            cairo_blur::blur_image_surface(&mut surface, 20);
        }
        surface
            .with_data(|s| {
                let mut img = RgbImage::new(s, w.w(), w.h(), ColorDepth::Rgba8).unwrap();
                img.draw(w.x(), w.y(), w.w(), w.h());
            })
            .unwrap();
        draw::set_draw_color(Color::Black);
        draw::set_font(Font::Helvetica, app::font_size());
        if !w.value() {
            draw::draw_rbox(
                w.x() + 1,
                w.y() + 1,
                w.w() - 6,
                w.h() - 6,
                15,
                true,
                Color::White,
            );
            draw::draw_text2(
                &w.label(),
                w.x() + 1,
                w.y() + 1,
                w.w() - 6,
                w.h() - 6,
                Align::Center,
            );
        } else {
            draw::draw_rbox(
                w.x() + 1,
                w.y() + 1,
                w.w() - 4,
                w.h() - 4,
                15,
                true,
                Color::White,
            );
            draw::draw_text2(
                &w.label(),
                w.x() + 1,
                w.y() + 1,
                w.w() - 4,
                w.h() - 4,
                Align::Center,
            );
        }
    });
    element
}

fn draw_surface(surface: &mut ImageSurface, w: i32, h: i32) {
    let ctx = Context::new(surface).unwrap();
    ctx.save().unwrap();
    let corner_radius = h as f64 / 10.0;
    let radius = corner_radius / 1.0;
    let degrees = std::f64::consts::PI / 180.0;

    ctx.new_sub_path();
    ctx.arc(w as f64 - radius, radius, radius, -90. * degrees, 0.0);
    ctx.arc(
        w as f64 - radius,
        h as f64 - radius,
        radius,
        0.0,
        90. * degrees,
    );
    ctx.arc(
        radius,
        h as f64 - radius,
        radius,
        90. * degrees,
        180. * degrees,
    );
    ctx.arc(radius, radius, radius, 180. * degrees, 270. * degrees);
    ctx.close_path();

    ctx.set_source_rgba(150.0 / 255.0, 150.0 / 255.0, 150.0 / 255.0, 40.0 / 255.0);
    ctx.set_line_width(4.);
    ctx.fill().unwrap();
    ctx.restore().unwrap();
}
