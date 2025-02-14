use std::sync::Arc;

use axum::extract::State;
use axum::Json;

use crate::database::Device;
use crate::{AppState, Result};

#[utoipa::path(get, path = "/device/list")]
pub async fn list(State(state): State<Arc<AppState>>) -> Result<Json<Vec<Device>>> {
    let device_names = Device::get_all(&state.database)
        .await
        .map_err(anyhow::Error::from)?;

    Ok(Json(device_names))
}
