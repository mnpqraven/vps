pub mod gacha;
pub mod health;

use axum::routing::get;
use gacha::pull_simulation;
use utoipa_axum::router::OpenApiRouter;

pub fn app_router() -> OpenApiRouter {
    OpenApiRouter::new()
        // `GET /` goes to `root`
        .route("/", get(root).post(root))
        .merge(health::router())
        .merge(pull_simulation::router())
}

// basic handler that responds with a static string
async fn root() -> &'static str {
    "Hello, World!"
}
