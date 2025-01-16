use sqlx::{PgPool, Result};
use uuid::Uuid;

pub struct Device {
    pub id: Uuid,
    pub name: String,
}

impl Device {
    pub async fn register(name: String, database: &PgPool) -> Result<Uuid> {
        sqlx::query_scalar!("INSERT INTO devices(name) VALUES($1) RETURNING id", name)
            .fetch_one(database)
            .await
    }

    pub async fn get_all_names(database: &PgPool) -> Result<Vec<String>> {
        sqlx::query_scalar!("SELECT name FROM devices")
            .fetch_all(database)
            .await
    }

    pub async fn get_token(name: String, database: &PgPool) -> Result<Uuid> {
        sqlx::query_scalar!("SELECT id FROM devices WHERE name = $1 LIMIT 1", name)
            .fetch_one(database)
            .await
    }
}
