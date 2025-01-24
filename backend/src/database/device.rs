use serde::Serialize;
use sqlx::{PgPool, Result};
use uuid::Uuid;

#[derive(Serialize)]
pub struct Device {
    pub id: Uuid,
    pub name: String,
    pub latitude: f32,
    pub longitude: f32,
}

impl Device {
    pub async fn register(
        name: String,
        latitude: f32,
        longitude: f32,
        database: &PgPool,
    ) -> Result<Uuid> {
        sqlx::query_scalar!(
            "INSERT INTO devices(name, latitude, longitude) VALUES($1, $2, $3) RETURNING id",
            name,
            latitude,
            longitude
        )
        .fetch_one(database)
        .await
    }

    pub async fn get_all(database: &PgPool) -> Result<Vec<Self>> {
        sqlx::query_as!(Self, "SELECT * FROM devices")
            .fetch_all(database)
            .await
    }
}
