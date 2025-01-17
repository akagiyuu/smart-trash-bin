use std::sync::Arc;

use axum::extract::State;
use axum::Json;
use axum_extra::headers::authorization::Bearer;
use axum_extra::headers::Authorization;
use axum_extra::TypedHeader;
use uuid::Uuid;

use crate::database::Status;
use crate::{AppState, Result};

async fn _post_data(state: Arc<AppState>, token: &str, status: Status) -> anyhow::Result<()> {
    let device_id = Uuid::parse_str(token)?;

    let _ = state.sender.send((device_id, status));

    status.insert(device_id, &state.database).await?;

    Ok(())
}

#[utoipa::path(
    post,
    path = "/data",
    request_body = Status,
    security(("token" = []))
)]
pub async fn post_data(
    State(state): State<Arc<AppState>>,
    TypedHeader(Authorization(bearer)): TypedHeader<Authorization<Bearer>>,
    Json(data): Json<Status>,
) -> Result<()> {
    _post_data(state, bearer.token(), data).await?;

    Ok(())
}
