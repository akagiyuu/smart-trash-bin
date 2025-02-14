use std::sync::Arc;

use axum::extract::State;
use axum::Json;
use uuid::Uuid;

use crate::database::Device;
use crate::{AppState, Result};

#[utoipa::path(
    post,
    path = "/register",
    request_body = String,
)]
pub async fn register(State(state): State<Arc<AppState>>, name: String) -> Result<Json<Uuid>> {
    let device_id = Device::register(name, &state.database)
        .await
        .map_err(anyhow::Error::from)?;

    Ok(Json(device_id))
}
