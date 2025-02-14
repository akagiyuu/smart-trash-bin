use std::str::FromStr;
use std::sync::Arc;

use axum::extract::State;
use axum::Json;
use uuid::Uuid;

use crate::database::Device;
use crate::{AppState, Result};

#[utoipa::path(
    post,
    path = "/register",
    request_body = Uuid,
)]
pub async fn register(
    State(state): State<Arc<AppState>>,
    id: String,
) -> Result<Json<Uuid>> {
    let id = Uuid::from_str(&id).map_err(anyhow::Error::from)?;
    let device_id = Device::register(id, &state.database)
        .await
        .map_err(anyhow::Error::from)?;

    Ok(Json(device_id))
}
