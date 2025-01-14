use std::sync::Arc;

use axum::extract::State;
use axum::Json;

use crate::server::{Data, DataKind};
use crate::{AppState, Result};

#[utoipa::path(
    post,
    path = "/receive",
    request_body = Data
)]
pub async fn receive(State(state): State<Arc<AppState>>, Json(data): Json<Data>) -> Result<()> {
    match data.kind {
        DataKind::Status(is_open) => {
            sqlx::query!(
                r#"
                    INSERT INTO statuses(time, is_open)
                    VALUES (?, ?)
                "#,
                data.time,
                is_open
            )
            .execute(&state.database)
            .await
        }
        DataKind::Moisture(moisture) => {
            sqlx::query!(
                r#"
                    INSERT INTO moistures(time, moisture)
                    VALUES (?, ?)
                "#,
                data.time,
                moisture
            )
            .execute(&state.database)
            .await
        }
        DataKind::TrashLevel(trash_level) => {
            sqlx::query!(
                r#"
                    INSERT INTO trash_levels(time, trash_level)
                    VALUES (?, ?)
                "#,
                data.time,
                trash_level
            )
            .execute(&state.database)
            .await
        }
    }
    .map_err(anyhow::Error::from)?;

    Ok(())
}
