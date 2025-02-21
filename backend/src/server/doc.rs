use utoipa::OpenApi;

use crate::database;

use super::handler;

#[derive(OpenApi)]
#[openapi(
    paths(
        handler::ping,
        handler::register::register,
        handler::list::list,
        handler::data::get_data,
        handler::data::post_data,
        handler::name::get_name,
        handler::name::post_name,
        handler::full::get_full,
    ),
    components(schemas(database::Status, handler::data::Data, handler::full::FullDevice))
)]
pub struct ApiDoc;
