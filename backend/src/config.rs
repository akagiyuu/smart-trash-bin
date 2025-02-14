use std::sync::LazyLock;

use serde::Deserialize;

const fn default_port() -> u16 {
    3000
}

const fn default_broadcast_channel_count() -> usize {
    512
}

fn default_public_cors_domain() -> String {
    "http://localhost:5173".to_string()
}

fn default_local_cors_domain() -> String {
    "http://localhost:5173".to_string()
}

fn default_trash_level_threshold() -> f32 {
    80.0
}

#[derive(Deserialize)]
pub struct Config {
    #[serde(default = "default_public_cors_domain")]
    pub public_cors_domain: String,

    #[serde(default = "default_local_cors_domain")]
    pub local_cors_domain: String,

    #[serde(default = "default_port")]
    pub port: u16,

    pub database_url: String,

    #[serde(default = "default_broadcast_channel_count")]
    pub broadcast_channel_count: usize,

    pub sender_email: String,

    pub sender_password: String,

    pub receiver_email: String,

    #[serde(default = "default_trash_level_threshold")]
    pub trash_level_threshold: f32,
}

pub static CONFIG: LazyLock<Config> = LazyLock::new(|| {
    ::config::Config::builder()
        .add_source(config::Environment::default())
        .build()
        .unwrap()
        .try_deserialize()
        .unwrap()
});
