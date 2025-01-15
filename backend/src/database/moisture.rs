use chrono::{DateTime, Utc};
use sqlx::{PgPool, Result};
use uuid::Uuid;

pub struct Moisture {
    pub device_id: Uuid,
    pub time: DateTime<Utc>,
    pub moisture: f32,
}

impl Moisture {
    pub async fn insert(self, database: &PgPool) -> Result<()> {
        sqlx::query!(
            "INSERT INTO moistures(device_id, time, moisture) VALUES($1, $2, $3)",
            self.device_id,
            self.time,
            self.moisture
        )
        .execute(database)
        .await?;

        Ok(())
    }
}
