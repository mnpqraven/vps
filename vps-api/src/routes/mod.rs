pub mod blog;
pub mod gacha;
pub mod health;
pub mod rpcgreet;

use axum::routing::get;
use utoipa_axum::router::OpenApiRouter;

pub fn app_router() -> OpenApiRouter {
    OpenApiRouter::new()
        // `GET /` goes to `root`
        .route("/", get(root).post(root))
        .nest("/api/health", health::router())
        .nest("/api/rpcgreet", rpcgreet::router())
        .nest("/api/gacha/pull_simulation", gacha::router())
        .nest("/api/blog/tag", blog::blog_tag::router())
}

// basic handler that responds with a static string
async fn root() -> &'static str {
    "Go to /swagger for docs"
}
