use utoipa::{
    openapi::security::{HttpAuthScheme, HttpBuilder, SecurityScheme},
    Modify, OpenApi,
};

use crate::database;

use super::handler;

struct SecurityAddon;

impl Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        if let Some(components) = openapi.components.as_mut() {
            components.add_security_scheme(
                "token",
                SecurityScheme::Http(HttpBuilder::new().scheme(HttpAuthScheme::Bearer).build()),
            )
        }
    }
}

#[derive(OpenApi)]
#[openapi(
    paths(
        handler::ping,
        handler::device::device,
        handler::get_data::get_data,
        handler::post_data::post_data,
        handler::register::register,
    ),
    modifiers(&SecurityAddon),
    components(schemas(database::Status))
)]
pub struct ApiDoc;
