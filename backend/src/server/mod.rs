use std::sync::Arc;

use axum::{
    routing::{get, post},
    Router,
};
use doc::ApiDoc;
use tower_http::trace::TraceLayer;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use crate::AppState;

mod doc;
mod handler;

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
