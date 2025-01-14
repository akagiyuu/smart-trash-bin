use std::sync::Arc;

use axum::{
    routing::{get, post},
    Router,
};
use chrono::{DateTime, NaiveDateTime, Utc};
use doc::ApiDoc;
use serde::Deserialize;
use tower_http::trace::TraceLayer;
use utoipa::{OpenApi, ToSchema};
use utoipa_swagger_ui::SwaggerUi;

use crate::AppState;

mod doc;
mod handler;

#[derive(Clone, Copy, Deserialize, ToSchema)]
#[serde(tag = "type", content = "data", rename_all = "snake_case")]
pub enum DataKind {
    Status(bool),
    Moisture(f64),
    TrashLevel(f64),
}

#[derive(Clone, Copy, Deserialize, ToSchema)]
pub struct Data {
    pub time: DateTime<Utc>,

    #[serde(flatten)]
    pub kind: DataKind,
}

pub fn build(state: Arc<AppState>) -> Router {
    // register routes
    let router = Router::new()
        .route("/", get(handler::ping))
        .route("/receive", post(handler::receive::receive));

    // add openapi doc and swagger
    let router = router
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()));

    // register global middlewares
    let router = router.layer(TraceLayer::new_for_http());

    // init state
    router.with_state(state)
}
