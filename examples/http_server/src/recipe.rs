use axum::{routing::get, Router, response::Html};
use std::{net::SocketAddr};
use tokio::sync::mpsc;
use futures::StreamExt;
use tokio_stream::wrappers::UnboundedReceiverStream;
use flemish::subscription::Recipe;
use futures::stream::BoxStream;

pub struct AxumServerRecipe {
    pub addr: String,
}

impl Recipe for AxumServerRecipe {
    type Output = String;

    fn stream(self: Box<Self>) -> BoxStream<'static, String> {
        let (tx, rx) = mpsc::unbounded_channel::<String>();
        let addr = self.addr.clone();

        let app = Router::new().route("/", get({
            let tx = tx.clone();
            move || {
                let _ = tx.send("Got request to /".into());
                async { Html("Hello from Axum + Flemish!") }
            }
        }));

        let s = async_stream::stream! {
            
            tokio::task::spawn(async move {
                let server = axum::serve(tokio::net::TcpListener::bind(&addr).await.unwrap(), app);
                if let Err(e) = server.await {
                    eprintln!("Axum server error: {}", e);
                }
            });

            let mut rx_stream = UnboundedReceiverStream::new(rx);

            while let Some(msg) = rx_stream.next().await {
                yield msg;
            }
        };

        s.boxed()
    }
}
