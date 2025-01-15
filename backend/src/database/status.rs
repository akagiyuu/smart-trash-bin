use chrono::{DateTime, Utc};
use sqlx::{PgPool, Result};
use uuid::Uuid;

pub struct Status {
    pub device_id: Uuid,
    pub time: DateTime<Utc>,
    pub is_open: bool,
}

impl Status {
    pub async fn insert(self, database: &PgPool) -> Result<()> {
        sqlx::query!(
            "INSERT INTO statuses(device_id, time, is_open) VALUES($1, $2, $3)",
            self.device_id,
            self.time,
            self.is_open
        )
        .execute(database)
        .await?;

        Ok(())
    }
}
