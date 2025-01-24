use std::sync::Arc;

use crate::AppState;
use axum::{
    extract::{
        ws::{Message, WebSocket},
        Path, State, WebSocketUpgrade,
    },
    response::IntoResponse,
};
use uuid::Uuid;

async fn handle_socket(state: Arc<AppState>, device_id: Uuid, mut socket: WebSocket) {
    let mut receiver = state.sender.subscribe();
    while let Ok((remote_device_id, status)) = receiver.recv().await {
        if remote_device_id != device_id {
            continue;
        }

        let json = match serde_json::to_string(&status) {
            Ok(v) => v,
            Err(error) => {
                tracing::error!("Failed to serialize {:?} with error {}", status, error);
                continue;
            }
        };
        if let Err(error) = socket.send(Message::text(json)).await {
            tracing::error!("Failed to send data to client {}", error);
        }
    }
}

#[utoipa::path(get, path = "/data/:device_id")]
pub async fn get_data(
    State(state): State<Arc<AppState>>,
    Path(device_id): Path<Uuid>,
    ws: WebSocketUpgrade,
) -> impl IntoResponse {
    ws.on_upgrade(move |socket| handle_socket(state, device_id, socket))
}
