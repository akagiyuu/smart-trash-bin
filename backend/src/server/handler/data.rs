use std::sync::Arc;

use axum::Json;
use axum::{
    extract::{
        ws::{Message, WebSocket},
        Path, State, WebSocketUpgrade,
    },
    response::IntoResponse,
};
use chrono::Utc;
use serde::Deserialize;
use utoipa::ToSchema;
use uuid::Uuid;

use crate::database::{Device, Status};
use crate::{util, AppState, Result, CONFIG};

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

#[utoipa::path(get, path = "/device/:device_id/data")]
pub async fn get_data(
    State(state): State<Arc<AppState>>,
    Path(device_id): Path<Uuid>,
    ws: WebSocketUpgrade,
) -> Result<impl IntoResponse> {
    Ok(ws.on_upgrade(move |socket| handle_socket(state, device_id, socket)))
}

#[derive(ToSchema, Deserialize)]
pub struct Data {
    pub is_open: bool,
    pub trash_level: f32,
}

#[utoipa::path(
    post,
    path = "/device/:device_id/data",
    request_body = Status,
)]
pub async fn post_data(
    State(state): State<Arc<AppState>>,
    Path(device_id): Path<Uuid>,
    Json(data): Json<Data>,
) -> Result<()> {
    let status = Status {
        time: Utc::now(),
        is_open: data.is_open,
        trash_level: data.trash_level,
    };

    let _ = state.sender.send((device_id, status));

    if data.trash_level > CONFIG.trash_level_threshold {
        util::send_email(
            format!(
                "Device {} exceed trash level threshold",
                Device::get_name(device_id, &state.database)
                    .await
                    .map_err(anyhow::Error::from)?
            ),
            format!("Current trash level: {}%", data.trash_level),
        )?;
    }

    status
        .insert(device_id, &state.database)
        .await
        .map_err(anyhow::Error::from)?;

    Ok(())
}
