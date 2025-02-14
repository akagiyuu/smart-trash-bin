use std::sync::Arc;

use axum::{
    http::{
        header::{
            ACCEPT, ACCESS_CONTROL_ALLOW_HEADERS, ACCESS_CONTROL_ALLOW_METHODS,
            ACCESS_CONTROL_ALLOW_ORIGIN, AUTHORIZATION, CONTENT_TYPE, ORIGIN,
        },
        HeaderName, HeaderValue, Method,
    },
    routing::{get, post},
    Router,
};
use doc::ApiDoc;
use tower_http::{cors::CorsLayer, trace::TraceLayer};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use crate::{AppState, CONFIG};

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
    let allow_origins = [
        CONFIG.public_cors_domain.parse::<HeaderValue>().unwrap(),
        CONFIG.local_cors_domain.parse::<HeaderValue>().unwrap(),
    ];

    // register routes
    let router = Router::new()
        .route("/", get(handler::ping))
        .route("/device", get(handler::device::device))
        .route("/data/{device_name}", get(handler::get_data::get_data))
        .route("/data", post(handler::post_data::post_data))
        .route("/register", post(handler::register::register));

    let router = router.layer(
        CorsLayer::new()
            .allow_origin(allow_origins)
            .allow_headers(ALLOW_HEADERS)
            .expose_headers(ALLOW_HEADERS)
            .allow_credentials(true)
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
