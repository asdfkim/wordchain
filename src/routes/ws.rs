use crate::model::AppState;
use axum::Router;
use axum::extract::ws::{Message, WebSocket};
use axum::extract::{State, WebSocketUpgrade};
use axum::response::IntoResponse;
use axum::routing::get;
use futures_util::{sink::SinkExt, stream::StreamExt};
use std::sync::Arc;

pub fn router() -> Router<Arc<AppState>> {
    Router::new().route("/", get(upgrade))
}

async fn upgrade(ws: WebSocketUpgrade, State(state): State<Arc<AppState>>) -> impl IntoResponse {
    ws.on_upgrade(|socket| handle(socket, state))
}

async fn handle(socket: WebSocket, state: Arc<AppState>) {
    let (mut sender, mut receiver) = socket.split();
    let mut rx = state.tx.subscribe();
    let tx = state.tx.clone();

    let send_task = tokio::spawn(async move {
        while let Ok(msg) = rx.recv().await {
            if sender.send(Message::Text(msg.into())).await.is_err() {
                break;
            }
        }
    });

    while let Some(Ok(Message::Text(text))) = receiver.next().await {
        let _ = tx.send(text.to_string());
    }

    send_task.abort();
}
