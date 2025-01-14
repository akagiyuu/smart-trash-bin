pub mod receive;
pub mod get_data;

#[utoipa::path(
    get,
    path = "/",
    responses(
        (status = 200, description = "Ping", body = &'static str),
    ),
)]
pub async fn ping() -> &'static str {
    "Ping"
}
