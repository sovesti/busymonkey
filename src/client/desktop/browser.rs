use std::time::Duration;

use axum::{
    Router,
    extract::State,
    extract::{WebSocketUpgrade, ws::Message},
    response::Response,
    routing::get,
};
use dioxus::hooks::use_future;
use dioxus::prelude::error;
use dioxus::prelude::use_context;
use dioxus::prelude::use_context_provider;
use tokio::sync::broadcast;

pub fn use_browser() {
    let channel = use_context_provider(|| broadcast::channel::<String>(32).0);
    use_future(move || {
        let _ch = channel.clone();
        async move {
            if let Err(err) = crate::client::desktop::browser::serve(_ch).await {
                error!("error initialising server for communication with extension: {err:?}")
            }
        }
    });
}
pub struct Sender {
    channel: broadcast::Sender<String>,
}
impl Sender {
    pub fn send(&self, message: &str) -> anyhow::Result<()> {
        self.channel.send(message.into())?;
        Ok(())
    }
}
pub fn use_sender() -> Sender {
    Sender {
        channel: use_context::<broadcast::Sender<String>>(),
    }
}

pub async fn serve(channel: broadcast::Sender<String>) -> anyhow::Result<()> {
    let listener = tokio::net::TcpListener::bind("127.0.0.1:17845").await?;
    axum::serve(
        listener,
        Router::new().route("/ws", get(connect)).with_state(channel),
    )
    .await?;
    Ok(())
}

async fn connect(
    State(channel): State<broadcast::Sender<String>>,
    ws: WebSocketUpgrade,
) -> Response {
    ws.on_upgrade(|mut socket| async move {
        let mut messages = channel.subscribe();
        let mut heartbeat = tokio::time::interval(Duration::from_secs(20));
        loop {
            let text = tokio::select! {
                message = messages.recv() => {
                    let Ok(text) = message else { break };
                    text
                }
                _ = heartbeat.tick() => String::new(),
            };
            if socket.send(Message::Text(text.into())).await.is_err() {
                break;
            }
        }
    })
}
