use utoipa::OpenApi;

use super::handler;

#[derive(OpenApi)]
#[openapi(
    paths(
        handler::ping,
        handler::receive::receive
    ),
    components(schemas(
        super::Data
    ))
)]
pub struct ApiDoc;
