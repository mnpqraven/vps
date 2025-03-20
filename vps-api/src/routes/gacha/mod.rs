use utoipa_axum::{router::OpenApiRouter, routes};

pub mod pull_simulation;

pub fn router() -> OpenApiRouter {
    OpenApiRouter::new().routes(routes!(pull_simulation::pull_simulation))
}
