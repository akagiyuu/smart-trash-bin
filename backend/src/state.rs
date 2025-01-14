use anyhow::Result;
use sqlx::SqlitePool;
use tokio::sync::broadcast;

use crate::{server::Data, CONFIG};

pub struct AppState {
    pub database: SqlitePool,
    pub sender: broadcast::Sender<Data>,
}

impl AppState {
    pub async fn new() -> Result<Self> {
        let database = SqlitePool::connect("sqlite:data.db").await?;
        let (sender, _) = broadcast::channel(CONFIG.broadcast_channel_count);

        Ok(Self { database, sender })
    }
}
