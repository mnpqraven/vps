pub mod gacha;

use axum::{routing::get, Router};
use gacha::pull_simulation;

pub fn app_router() -> Router {
    Router::new()
        // `GET /` goes to `root`
        .route("/", get(root).post(root))
        .route(
            "/gacha/pull_simulation",
            get(pull_simulation::handle).post(pull_simulation::handle),
        )
}

// basic handler that responds with a static string
async fn root() -> &'static str {
    "Hello, World!"
}
