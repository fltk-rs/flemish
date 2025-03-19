use fltk::prelude::*;
use fltk::*;
use std::cell::RefCell;
use std::rc::Rc;

use crate::id;
use crate::settings::Settings;
use crate::subscription::*;
use crate::task::Task;
use crate::vdom;
use crate::view::View;

pub struct Application<T, Message: Send + Sync, U: Into<Task<Message>>> {
    title: String,
    update_fn: fn(&mut T, Message) -> U,
    view_fn: fn(&T) -> View<Message>,
    settings: Option<Settings<Message>>,
    subscription: Option<fn(&T) -> Subscription<Message>>,
}

impl<T, Message: Clone + Send + Sync + 'static, U: Into<Task<Message>>> Application<T, Message, U> {
    pub fn new(
        title: &str,
        update_fn: fn(&mut T, Message) -> U,
        view_fn: fn(&T) -> View<Message>,
    ) -> Self {
        Self {
            title: title.to_string(),
            update_fn,
            view_fn,
            settings: None,
            subscription: None,
        }
    }
    pub fn settings(mut self, settings: Settings<Message>) -> Self {
        self.settings = Some(settings);
        self
    }
    pub fn subscription(mut self, subscription_fn: fn(&T) -> Subscription<Message>) -> Self {
        self.subscription = Some(subscription_fn);
        self
    }

    fn update(&mut self, t: &mut T, message: Message) -> Task<Message> {
        (self.update_fn)(t, message).into()
    }

    fn view_(&self, t: &T) -> View<Message> {
        let mut v = (self.view_fn)(t);
        id::reset_id();
        v.assign_ids_topdown();
        v
    }
    fn init(&self) -> (app::App, window::Window, tokio::runtime::Runtime) {
        let a = app::App::default();
        let binding = Settings::default();
        let settings: &Settings<Message> = self.settings.as_ref().unwrap_or(&binding);
        if let Some(color_map) = settings.color_map {
            fltk_theme::ColorTheme::from_colormap(color_map).apply();
        } else {
            fltk_theme::ColorTheme::from_colormap(fltk_theme::color_themes::BLACK_THEME).apply();
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
        if let Some(theme) = settings.theme {
            fltk_theme::WidgetTheme::new(theme).apply();
        }

        if let Some(scheme) = settings.scheme {
            app::set_scheme(scheme);
        }
        if let Some(font) = settings.font {
            app::set_font(font);
        }
        if let Some(ms) = settings.menu_linespacing {
            app::set_menu_linespacing(ms);
        }
        let (w, h) = settings.size;
        let w = if w == 0 { 400 } else { w };
        let h = if h == 0 { 300 } else { h };
        let (x, y) = settings.pos;
        let mut win = window::Window::default()
            .with_size(w, h)
            .with_label(&self.title);
        win.set_xclass(&self.title);
        if (x, y) != (0, 0) {
            win.set_pos(x, y);
        }
        if let Some((min_w, min_h, max_w, max_h)) = settings.size_range {
            win.size_range(min_w, min_h, max_w, max_h);
        }

        if let Some(close) = settings.on_close.clone() {
            win.set_callback(move |_| {
                app::Sender::<Message>::get().send(close.clone());
            });
        } else if settings.ignore_esc_close {
            win.set_callback(move |_| {
                if app::event() == enums::Event::Close {
                    app::quit();
                }
            });
        }
        win.make_resizable(settings.resizable);
        let rt = if let Some(worker_threads) = settings.worker_threads {
            if worker_threads == 0 {
                tokio::runtime::Builder::new_current_thread()
                    .enable_all()
                    .build()
                    .unwrap()
            } else {
                tokio::runtime::Builder::new_multi_thread()
                    .worker_threads(worker_threads)
                    .enable_all()
                    .build()
                    .unwrap()
            }
        } else {
            tokio::runtime::Runtime::new().unwrap()
        };
        (a, win, rt)
    }
    pub fn run_with<F: Fn() -> T>(self, init_fn: F) {
        let (a, mut win, rt) = self.init();

        let mut t = init_fn();
        let initial_vdom = self.view_(&t);
        let vdom = vdom::VirtualDom::new(initial_vdom);

        win.end();
        win.show();

        if let Some(mut first_child) = win.child(0) {
            first_child.resize(0, 0, win.w(), win.h());
        }

        let (s, r) = app::channel::<Message>();
        rt.block_on(async {
            if let Some(subscription) = self.subscription {
                start_subscription(&mut win, vec![subscription(&t)], s.clone());
            }

            let state = Rc::new(RefCell::new(self));

            while a.wait() {
                if let Some(msg) = r.recv() {
                    let mut st = state.borrow_mut();
                    vdom.dispatch(msg.clone());
                    let command = st.update(&mut t, msg.clone());

                    command.execute(s.clone());
                    let new_vdom = st.view_(&t);

                    vdom.patch(new_vdom);
                    app::redraw();
                }
            }
        });
    }
    pub fn run(self)
    where
        T: Default,
    {
        self.run_with(T::default);
    }
}
