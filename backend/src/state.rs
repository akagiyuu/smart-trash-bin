use anyhow::Result;
use sqlx::PgPool;
use tokio::sync::broadcast;
use uuid::Uuid;

use crate::{database::Status, CONFIG};

pub struct AppState {
    pub database: PgPool,
    pub sender: broadcast::Sender<(Uuid, Status)>,
}

impl AppState {
    pub async fn new() -> Result<Self> {
        let database = PgPool::connect(&CONFIG.database_url).await?;
        let (sender, _) = broadcast::channel(CONFIG.broadcast_channel_count);

        Ok(Self { database, sender })
    }
}
