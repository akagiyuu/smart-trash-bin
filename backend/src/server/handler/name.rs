use std::sync::Arc;

use axum::extract::{Path, State};
use uuid::Uuid;

use crate::database::Device;
use crate::{AppState, Result};

#[utoipa::path(get, path = "/name/:device_id")]
pub async fn get_name(
    State(state): State<Arc<AppState>>,
    Path(device_id): Path<Uuid>,
) -> Result<String> {
    let device_name = Device::get_name(device_id, &state.database)
        .await
        .map_err(anyhow::Error::from)?;

    Ok(device_name)
}

#[utoipa::path(post, path = "/name/:device_id", request_body = String)]
pub async fn post_name(
    State(state): State<Arc<AppState>>,
    Path(device_id): Path<Uuid>,
    name: String,
) -> Result<()> {
    Device::change_name(device_id, name, &state.database)
        .await
        .map_err(anyhow::Error::from)?;

    Ok(())
}
