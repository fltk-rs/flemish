#![forbid(unsafe_code)]

mod model;

use {
    cairo::{Context, Format, ImageSurface},
    flemish::{
        app,
        button::Button,
        color_themes, draw,
        enums::{Align, Color, ColorDepth, Event, Font, Shortcut},
        frame::Frame,
        group::Flex,
        image::RgbImage,
        menu::{MenuButton, MenuButtonType, MenuFlag},
        prelude::*,
        OnEvent, OnMenuEvent, Sandbox, Settings,
    },
    model::Model,
};

#[derive(Clone, Copy)]
pub enum Message {
    Inc,
    Dec,
    Quit,
}

const NAME: &str = "FlCairoButton";

fn main() {
    Model::new().run(Settings {
        size: (640, 360),
        ignore_esc_close: true,
        resizable: false,
        background: Some(Color::from_u32(0xfdf6e3)),
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
        format!("{} - {NAME}", self.value())
    }

    fn view(&mut self) {
        let mut page = Flex::default()
            .with_size(600, 200)
            .center_of_parent()
            .column();

        let hero = Flex::default(); //HERO
        crate::cairobutton()
            .with_label("@#<")
            .on_event(move |_| Message::Dec);
        crate::frame()
            .with_label(&self.value())
            .handle(crate::popup);
        crate::cairobutton()
            .with_label("@#>")
            .on_event(move |_| Message::Inc);
        hero.end();

        page.end();
        page.set_pad(0);
        page.set_margin(0);
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::Inc => self.inc(),
            Message::Dec => self.dec(),
            Message::Quit => {
                self.save();
                app::quit();
            }
        }
    }
}

fn menu() -> MenuButton {
    MenuButton::default()
        .with_type(MenuButtonType::Popup3)
        .on_item_event(
            "@#+  &Increment",
            Shortcut::Ctrl | 'i',
            MenuFlag::Normal,
            move |_| Message::Inc,
        )
        .on_item_event(
            "@#-  &Decrement",
            Shortcut::Ctrl | 'd',
            MenuFlag::Normal,
            move |_| Message::Dec,
        )
        .on_item_event(
            "@#1+  Quit",
            Shortcut::Ctrl | 'q',
            MenuFlag::Normal,
            move |_| Message::Quit,
        )
}

fn popup(_: &mut Frame, event: Event) -> bool {
    match event {
        Event::Push => match app::event_mouse_button() {
            app::MouseButton::Right => {
                crate::menu().popup();
                true
            }
            _ => false,
        },
        _ => false,
    }
}

fn frame() -> Frame {
    let mut element = Frame::default();
    element.set_label_size(60);
    element
}

fn cairobutton() -> Button {
    let mut element = Button::default();
    element.super_draw(false);
    element.draw(|button| {
        draw::draw_rect_fill(
            button.x(),
            button.y(),
            button.w(),
            button.h(),
            Color::from_u32(0xfdf6e3),
        );
        let mut surface = ImageSurface::create(Format::ARgb32, button.w(), button.h())
            .expect("Couldn’t create surface");
        crate::draw_surface(&mut surface, button.w(), button.h());
        if !button.value() {
            cairo_blur::blur_image_surface(&mut surface, 20);
        }
        surface
            .with_data(|surface| {
                RgbImage::new(surface, button.w(), button.h(), ColorDepth::Rgba8)
                    .unwrap()
                    .draw(button.x(), button.y(), button.w(), button.h());
            })
            .unwrap();
        draw::set_draw_color(Color::Black);
        draw::set_font(Font::Helvetica, app::font_size());
        if !button.value() {
            draw::draw_rbox(
                button.x() + 1,
                button.y() + 1,
                button.w() - 6,
                button.h() - 6,
                15,
                true,
                Color::White,
            );
            draw::draw_text2(
                &button.label(),
                button.x() + 1,
                button.y() + 1,
                button.w() - 6,
                button.h() - 6,
                Align::Center,
            );
        } else {
            draw::draw_rbox(
                button.x() + 1,
                button.y() + 1,
                button.w() - 4,
                button.h() - 4,
                15,
                true,
                Color::White,
            );
            draw::draw_text2(
                &button.label(),
                button.x() + 1,
                button.y() + 1,
                button.w() - 4,
                button.h() - 4,
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
