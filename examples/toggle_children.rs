use flemish::{view::*, Settings};

fn main() {
    flemish::application("toggle_children", App::update, App::view)
        .settings(Settings {
            size: (260, 120),
            resizable: true,
            ..Default::default()
        })
        .run();
}

#[derive(Default)]
struct App {
    show_extra: bool,
}

#[derive(Clone, Copy)]
enum Msg {
    Toggle,
}

impl App {
    fn update(&mut self, msg: Msg) {
        match msg {
            Msg::Toggle => self.show_extra = !self.show_extra,
        }
    }

    fn view(&self) -> View<Msg> {
        let mut children: Vec<View<Msg>> = vec![Button::new("Toggle", Msg::Toggle).view()];
        if self.show_extra {
            children
                .push(Row::new(&[Frame::new("Extra").view(), Frame::new("Pane").view()]).view());
        }
        Column::new(&children).view()
    }
}
