use utoipa::OpenApi;

use super::handler;

#[derive(OpenApi)]
#[openapi(
    paths(
        handler::ping,
        handler::get_data::get_data,
        handler::post_data::post_data,
        handler::register::register,
    ),
    components(schemas(super::Data))
)]
pub struct ApiDoc;
