use serde::Serialize;
use sqlx::{PgPool, Result};
use uuid::Uuid;

#[derive(Serialize)]
pub struct Device {
    pub id: Uuid,
    pub name: String,
}

impl Device {
    pub async fn register(id: Uuid, database: &PgPool) -> Result<Uuid> {
        sqlx::query_scalar!(
            "INSERT INTO devices(id, name) VALUES($1, $2)",
            id,
            id.to_string()
        )
        .execute(database)
        .await?;

        Ok(id)
    }

    pub async fn get_name(id: Uuid, database: &PgPool) -> Result<String> {
        sqlx::query_scalar!("SELECT name FROM devices WHERE id = $1 LIMIT 1", id)
            .fetch_one(database)
            .await
    }

    pub async fn change_name(id: Uuid, name: String, database: &PgPool) -> Result<()> {
        sqlx::query!("UPDATE devices SET name = $2 WHERE id = $1", id, name)
            .execute(database)
            .await?;

        Ok(())
    }

    pub async fn get_all(database: &PgPool) -> Result<Vec<Self>> {
        sqlx::query_as!(Self, "SELECT id, name FROM devices")
            .fetch_all(database)
            .await
    }
}
