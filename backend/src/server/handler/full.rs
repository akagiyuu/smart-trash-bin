use std::sync::Arc;

use axum::{
    extract::{
        ws::{Message, WebSocket},
        State, WebSocketUpgrade,
    },
    response::IntoResponse,
};
use serde::Serialize;
use utoipa::ToSchema;
use uuid::Uuid;

use crate::{AppState, Result};

#[derive(Serialize, ToSchema)]
pub struct FullDevice {
    id: Uuid,
    trash_level: f32,
}

async fn handle_socket(state: Arc<AppState>, mut socket: WebSocket) {
    let mut receiver = state.sender.subscribe();
    while let Ok((id, status)) = receiver.recv().await {
        let device = FullDevice {
            id,
            trash_level: status.trash_level,
        };

        let json = match serde_json::to_string(&device) {
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

#[utoipa::path(get, path = "/device/full")]
pub async fn get_full(
    State(state): State<Arc<AppState>>,
    ws: WebSocketUpgrade,
) -> Result<impl IntoResponse> {
    Ok(ws.on_upgrade(move |socket| handle_socket(state, socket)))
}
