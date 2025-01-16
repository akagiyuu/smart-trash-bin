use chrono::{DateTime, Utc};
use sqlx::{PgPool, Result};
use uuid::Uuid;

pub struct Status {
    pub device_id: Uuid,
    pub time: DateTime<Utc>,
    pub is_open: bool,
    pub moisture: f32,
    pub trash_level: f32
}

impl Status {
    pub async fn insert(self, database: &PgPool) -> Result<()> {
        sqlx::query!(
            r#"
                INSERT INTO statuses(device_id, time, is_open, moisture, trash_level)
                VALUES($1, $2, $3, $4, $5)
            "#,
            self.device_id,
            self.time,
            self.is_open,
            self.moisture,
            self.trash_level
        )
        .execute(database)
        .await?;

        Ok(())
    }
}
