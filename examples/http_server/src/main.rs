use flemish::{theme::color_themes, view::*, Settings, Subscription};
use std::net::SocketAddr;
use std::time::Instant;

mod recipe;
use recipe::AxumServerRecipe;

fn main() {
    flemish::application("Axum Demo", App::update, App::view)
        .settings(Settings {
            size: (400, 200),
            color_map: Some(color_themes::BLACK_THEME),
            ..Default::default()
        })
        .subscription(App::subscription)
        .run_with(App::new);
}

struct App {
    last_req: String,
}

#[derive(Debug, Clone)]
enum Message {
    ServerHit(String),
}

impl App {
    fn new() -> Self {
        Self {
            last_req: "No requests yet!".into(),
        }
    }

    fn subscription(&self) -> Subscription<Message> {
        let recipe = AxumServerRecipe {
            addr: String::from("0.0.0.0:3000"),
        };
        Subscription::from_recipe(recipe).map(Message::ServerHit)
    }

    fn update(&mut self, msg: Message) {
        match msg {
            Message::ServerHit(request_str) => {
                println!("HTTP request => {}", request_str);
                self.last_req = request_str;
            }
        }
    }

    fn view(&self) -> View<Message> {
        Frame::new(&format!("Last request: {}", self.last_req)).view()
    }
}