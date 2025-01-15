use std::sync::Arc;

use axum::extract::State;
use axum::Json;
use axum_extra::headers::authorization::Bearer;
use axum_extra::headers::Authorization;
use axum_extra::TypedHeader;
use uuid::Uuid;

use crate::database::{Moisture, Status, TrashLevel};
use crate::server::{Data, DataKind};
use crate::{AppState, Result};

async fn _post_data(state: Arc<AppState>, token: &str, data: Data) -> anyhow::Result<()> {
    let device_id = Uuid::parse_str(token)?;

    state.sender.send((device_id, data))?;

    let time = data.time;

    match data.kind {
        DataKind::Status(is_open) => {
            Status {
                device_id,
                time,
                is_open,
            }
            .insert(&state.database)
            .await?
        }
        DataKind::Moisture(moisture) => {
            Moisture {
                device_id,
                time,
                moisture,
            }
            .insert(&state.database)
            .await?
        }
        DataKind::TrashLevel(trash_level) => {
            TrashLevel {
                device_id,
                time,
                trash_level,
            }
            .insert(&state.database)
            .await?
        }
    }

    Ok(())
}

#[utoipa::path(
    post,
    path = "/data",
    request_body = Data
)]
pub async fn post_data(
    State(state): State<Arc<AppState>>,
    TypedHeader(Authorization(bearer)): TypedHeader<Authorization<Bearer>>,
    Json(data): Json<Data>,
) -> Result<()> {
    _post_data(state, bearer.token(), data).await?;

    Ok(())
}
