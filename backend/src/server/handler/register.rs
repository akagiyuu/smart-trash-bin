use std::sync::Arc;

use axum::extract::State;
use axum::Json;
use serde::Deserialize;
use utoipa::ToSchema;
use uuid::Uuid;

use crate::database::Device;
use crate::{AppState, Result};

#[derive(Deserialize, ToSchema)]
pub struct RegisterData {
    name: String,
    latitude: f32,
    longitude: f32,
}

#[utoipa::path(
    post,
    path = "/register",
    request_body = RegisterData,
)]
pub async fn register(
    State(state): State<Arc<AppState>>,
    Json(data): Json<RegisterData>,
) -> Result<Json<Uuid>> {
    let device_id = Device::register(data.name, data.latitude, data.longitude, &state.database)
        .await
        .map_err(anyhow::Error::from)?;

    Ok(Json(device_id))
}
