use axum::{response::Html, routing::get, Router, extract::Path};
use flemish::{
    theme::color_themes, view::*, Settings, Subscription,
};

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
        Subscription::run_async(launch_server).map(Message::ServerHit)
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

async fn launch_server(tx: tokio::sync::mpsc::UnboundedSender<String>) {
    let app = Router::new().route(
        "/",
        get({
            let tx = tx.clone();
            move || {
                let _ = tx.send("Got request to /".into());
                async { Html("Hello from Axum + Flemish!\n") }
            }
        }),
    ).route(
        "/{p}",
        get({
            let tx = tx.clone();
            move |path: Path<String>| {
                let _ = tx.send(format!("Got request to /{}", path.0));
                async { Html("Hello from Axum + Flemish!\n") }
            }
        })
    );
    axum::serve(
        tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap(),
        app,
    )
    .await
    .unwrap();
}
