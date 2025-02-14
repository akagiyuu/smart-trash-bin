use std::{net::SocketAddr, sync::Arc};

use tracing::level_filters::LevelFilter;
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

pub mod config;
pub mod database;
pub mod error;
pub mod server;
pub mod state;
pub mod util;

pub use config::*;
pub use error::*;
pub use state::*;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::fmt::layer()
                .pretty()
                .with_timer(fmt::time::ChronoLocal::rfc_3339()),
        )
        .with(
            EnvFilter::builder()
                .with_default_directive(LevelFilter::DEBUG.into())
                .with_env_var("RUST_LOG")
                .from_env_lossy(),
        )
        .init();

    let state = Arc::new(AppState::new().await?);
    let app = server::build(state);

    let addr = SocketAddr::from(([127, 0, 0, 1], CONFIG.port));
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

    axum::serve(listener, app).await?;

    Ok(())
}
