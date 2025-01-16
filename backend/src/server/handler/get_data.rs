use std::sync::Arc;

use crate::{database::Device, AppState, Error, Result};
use anyhow::anyhow;
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
    while let Ok((remote_device_id, data)) = receiver.recv().await {
        if remote_device_id != device_id {
            continue;
        }

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
    path = "/data/:device_name",
)]
pub async fn get_data(
    State(state): State<Arc<AppState>>,
    Path(device_name): Path<String>,
    ws: WebSocketUpgrade,
) -> Result<impl IntoResponse> {
    let device_id = Device::get_token(device_name, &state.database)
        .await
        .map_err(|_| {
            Error::bad_request(
                "No device with given name",
                anyhow!("No device with given name"),
            )
        })?;

    Ok(ws.on_upgrade(move |socket| handle_socket(state, device_id, socket)))
}
