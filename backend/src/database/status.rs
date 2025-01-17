use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{PgPool, Result};
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, ToSchema)]
pub struct Status {
    pub time: DateTime<Utc>,
    pub is_open: bool,
    pub trash_level: f32,
}

impl Status {
    pub async fn insert(self, device_id: Uuid, database: &PgPool) -> Result<()> {
        sqlx::query!(
            r#"
                INSERT INTO statuses(device_id, time, is_open, trash_level)
                VALUES($1, $2, $3, $4)
            "#,
            device_id,
            self.time,
            self.is_open,
            self.trash_level
        )
        .execute(database)
        .await?;

        Ok(())
    }
}
