pub mod data;
pub mod list;
pub mod name;
pub mod register;
pub mod full;

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
