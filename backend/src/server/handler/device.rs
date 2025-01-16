use std::sync::Arc;

use axum::extract::State;
use axum::Json;

use crate::database::Device;
use crate::{AppState, Result};

#[utoipa::path(get, path = "/device")]
pub async fn device(State(state): State<Arc<AppState>>) -> Result<Json<Vec<String>>> {
    let device_names = Device::get_all_names(&state.database)
        .await
        .map_err(anyhow::Error::from)?;

    Ok(Json(device_names))
}
