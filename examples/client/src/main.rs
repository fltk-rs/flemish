use flemish::{view::*, Settings, Task};

fn main() {
    flemish::application("Axum Demo", App::update, App::view)
        .settings(Settings {
            size: (400, 400),
            ..Default::default()
        })
        .run();
}

#[derive(Default)]
struct App {
    content: String,
}

#[derive(Debug, Clone)]
enum Message {
    Fetch,
    Fetched(String),
}

impl App {
    fn update(&mut self, msg: Message) -> Task<Message> {
        match msg {
            Message::Fetch => Task::perform_async(|| async {
                reqwest::get("https://www.example.com")
                    .await
                    .unwrap()
                    .text()
                    .await
                    .unwrap()
            })
            .map(Message::Fetched),
            Message::Fetched(s) => {
                self.content = s.clone();
                Task::none()
            }
        }
    }

    fn view(&self) -> View<Message> {
        Column::new(&[
            HelpView::new(&self.content).view(),
            Button::new("Fetch", Message::Fetch).fixed(40).view(),
        ])
        .view()
    }
}
