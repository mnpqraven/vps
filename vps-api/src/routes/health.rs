use utoipa_axum::{router::OpenApiRouter, routes};

/// Get health of the API.
#[utoipa::path(
    method(get, head),
    path = "/api/health",
    responses(
        (status = OK, description = "Success", body = str, content_type = "text/plain")
    )
)]
async fn health() -> &'static str {
    "ok"
}
/// expose the Customer OpenAPI to parent module
pub fn router() -> OpenApiRouter {
    OpenApiRouter::new().routes(routes!(health))
}
