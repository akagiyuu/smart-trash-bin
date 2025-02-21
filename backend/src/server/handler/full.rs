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

use crate::{database::Device, AppState, Result, CONFIG};

#[derive(Serialize, ToSchema)]
pub struct FullDevice {
    name: String,
    trash_level: f32,
}

async fn handle_socket(state: Arc<AppState>, mut socket: WebSocket) -> anyhow::Result<()> {
    let mut receiver = state.sender.subscribe();
    while let Ok((id, status)) = receiver.recv().await {
        if status.trash_level <= CONFIG.trash_level_threshold {
            continue;
        }

        let name = Device::get_name(id, &state.database).await?;

        let device = FullDevice {
            name,
            trash_level: status.trash_level,
        };

        let json = serde_json::to_string(&device)?;
        socket.send(Message::text(json)).await?;
    }

    Ok(())
}

#[utoipa::path(get, path = "/device/full")]
pub async fn get_full(
    State(state): State<Arc<AppState>>,
    ws: WebSocketUpgrade,
) -> Result<impl IntoResponse> {
    Ok(ws.on_upgrade(|socket| async move {
        let _ = handle_socket(state, socket).await;
    }))
}
