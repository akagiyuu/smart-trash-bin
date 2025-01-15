use chrono::{DateTime, Utc};
use sqlx::{PgPool, Result};
use uuid::Uuid;

pub struct TrashLevel {
    pub device_id: Uuid,
    pub time: DateTime<Utc>,
    pub trash_level: f32,
}

impl TrashLevel {
    pub async fn insert(self, database: &PgPool) -> Result<()> {
        sqlx::query!(
            "INSERT INTO trash_levels(device_id, time, trash_level) VALUES($1, $2, $3)",
            self.device_id,
            self.time,
            self.trash_level
        )
        .execute(database)
        .await?;

        Ok(())
    }
}
