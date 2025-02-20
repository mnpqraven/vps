pub mod gacha;

use axum::{routing::get, Router};

pub fn app_router() -> Router {
    Router::new()
        // `GET /` goes to `root`
        .route("/", get(root))
        .route("/gacha/pull_simulation", get(gacha::pull_simulation::handle))
}

// basic handler that responds with a static string
async fn root() -> &'static str {
    "Hello, World!"
}
