use std::{sync::LazyLock, time::Duration};

use axum::{
    Router,
    extract::{WebSocketUpgrade, ws::Message},
    http::{HeaderMap, StatusCode},
    response::{IntoResponse, Response},
    routing::get,
};
use tokio::sync::broadcast;

static MESSAGES: LazyLock<broadcast::Sender<String>> = LazyLock::new(|| broadcast::channel(32).0);

pub fn send(message: &str) -> anyhow::Result<()> {
    MESSAGES
        .send(message.into())?;
    Ok(())
}

pub async fn serve() -> anyhow::Result<()> {
    let listener = tokio::net::TcpListener::bind("127.0.0.1:17845").await?;
    axum::serve(listener, Router::new().route("/ws", get(connect))).await?;
    Ok(())
}

async fn connect(headers: HeaderMap, ws: WebSocketUpgrade) -> Response {
    ws.on_upgrade(|mut socket| async move {
        let mut messages = MESSAGES.subscribe();
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
