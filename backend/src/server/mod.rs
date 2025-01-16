use std::sync::Arc;

use axum::{
    routing::{get, post},
    Router,
};
use chrono::{DateTime, Utc};
use doc::ApiDoc;
use serde::{Deserialize, Serialize};
use tower_http::trace::TraceLayer;
use utoipa::{OpenApi, ToSchema};
use utoipa_swagger_ui::SwaggerUi;

use crate::AppState;

mod doc;
mod handler;

#[derive(Clone, Copy, Deserialize, Serialize, ToSchema, Debug)]
#[serde(tag = "type", content = "data", rename_all = "snake_case")]
pub enum DataKind {
    Status(bool),
    Moisture(f32),
    TrashLevel(f32),
}

#[derive(Clone, Copy, Deserialize, Serialize, ToSchema, Debug)]
pub struct Data {
    pub time: DateTime<Utc>,

    #[serde(flatten)]
    pub kind: DataKind,
}

pub fn build(state: Arc<AppState>) -> Router {
    // register routes
    let router = Router::new()
        .route("/", get(handler::ping))
        .route("/device", get(handler::device::device))
        .route("/data/{device_name}", get(handler::get_data::get_data))
        .route("/data", post(handler::post_data::post_data))
        .route("/register", post(handler::register::register));

    // add openapi doc and swagger
    let router =
        router.merge(SwaggerUi::new("/swagger").url("/api-docs/openapi.json", ApiDoc::openapi()));

    // register global middlewares
    let router = router.layer(TraceLayer::new_for_http());

    // init state
    router.with_state(state)
}
