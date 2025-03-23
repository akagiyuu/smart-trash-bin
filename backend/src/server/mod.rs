use std::sync::Arc;

use axum::{
    http::{
        header::{
            ACCEPT, ACCESS_CONTROL_ALLOW_HEADERS, ACCESS_CONTROL_ALLOW_METHODS,
            ACCESS_CONTROL_ALLOW_ORIGIN, AUTHORIZATION, CONTENT_TYPE, ORIGIN,
        },
        HeaderName, Method,
    },
    routing::{get, post},
    Router,
};
use doc::ApiDoc;
use tower_http::{cors::{Any, CorsLayer}, trace::TraceLayer};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use crate::AppState;

mod doc;
mod handler;

const ALLOW_HEADERS: [HeaderName; 7] = [
    ORIGIN,
    AUTHORIZATION,
    ACCESS_CONTROL_ALLOW_ORIGIN,
    CONTENT_TYPE,
    ACCEPT,
    ACCESS_CONTROL_ALLOW_METHODS,
    ACCESS_CONTROL_ALLOW_HEADERS,
];
const ALLOW_METHODS: [Method; 2] = [Method::GET, Method::POST];

pub fn build(state: Arc<AppState>) -> Router {
    // register routes
    let router = Router::new()
        .route("/", get(handler::ping))
        .route("/device/register", post(handler::register::register))
        .route("/device/list", get(handler::list::list))
        .route("/device/{device_id}/data", get(handler::data::get_data))
        .route("/device/data", post(handler::data::post_data))
        .route("/device/{device_id}/name", get(handler::name::get_name))
        .route("/device/{device_id}name", post(handler::name::post_name))
        .route("/device/full", get(handler::full::get_full));

    let router = router.layer(
        CorsLayer::new()
            .allow_origin(Any)
            .allow_headers(ALLOW_HEADERS)
            .expose_headers(ALLOW_HEADERS)
            .allow_methods(ALLOW_METHODS),
    );

    // add openapi doc and swagger
    let router =
        router.merge(SwaggerUi::new("/swagger").url("/api-docs/openapi.json", ApiDoc::openapi()));

    // register global middlewares
    let router = router.layer(TraceLayer::new_for_http());

    // init state
    router.with_state(state)
}
