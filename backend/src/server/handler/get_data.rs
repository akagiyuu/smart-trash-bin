use std::sync::Arc;

use crate::AppState;
use axum::{
    extract::{
        ws::{Message, WebSocket},
        State, WebSocketUpgrade,
    },
    response::IntoResponse,
};

async fn handle_socket(state: Arc<AppState>, mut socket: WebSocket) {
    let mut receiver = state.sender.subscribe();
    while let Ok(data) = receiver.recv().await {
        let json = match serde_json::to_string(&data) {
            Ok(v) => v,
            Err(error) => {
                tracing::error!("Failed to serialize {:?} with error {}", data, error);
                continue;
            }
        };
        if let Err(error) = socket.send(Message::text(json)).await {
            tracing::error!("Failed to send data to client {}", error);
        }
    }
}

#[utoipa::path(
    get,
    path = "/data",
)]
pub async fn get_data(State(state): State<Arc<AppState>>, ws: WebSocketUpgrade) -> impl IntoResponse {
    ws.on_upgrade(move |socket| handle_socket(state, socket))
}
