pub mod auth;
pub mod blog;
pub mod gacha;
pub mod health;
pub mod rpcgreet;

use crate::{middlewares::github::oauth_client, utils::state::AppState};
use axum::{Extension, routing::get};
use oauth2::reqwest;
use utoipa::OpenApi;
use utoipa_axum::router::OpenApiRouter;
use utoipa_rapidoc::RapiDoc;
use utoipa_redoc::{Redoc, Servable};
use utoipa_swagger_ui::SwaggerUi;

#[derive(OpenApi)]
#[openapi()]
struct ApiDoc;

pub fn make_app_router() -> axum::Router {
    let (router, api) = OpenApiRouter::with_openapi(ApiDoc::openapi())
        .merge(app_router())
        .split_for_parts();

    let swagger = SwaggerUi::new("/swagger").url("/api/openapi-swagger.json", api.clone());
    let redoc = Redoc::with_url("/redoc", api.clone());
    let rapidoc = RapiDoc::with_url("/rapidoc", "/api/openapi-rapidoc.json", api.clone());

    router.merge(swagger).merge(redoc).merge(rapidoc)
}

pub fn app_router() -> OpenApiRouter {
    let state = AppState::new();
    // TODO: unwrap
    // there should be a cleaner way
    let auth_client = oauth_client().unwrap();
    let http_client = reqwest::ClientBuilder::new()
        // Following redirects opens the client up to SSRF vulnerabilities.
        .redirect(reqwest::redirect::Policy::none())
        .build()
        .expect("Client should build");

    OpenApiRouter::new()
        // `GET /` goes to `root`
        .route("/", get(root).post(root))
        .route("/auth/login", get(auth::login))
        .route("/auth/github_callback", get(auth::github_callback))
        .layer(Extension(auth_client))
        .layer(Extension(http_client))
        .with_state(state)
        .nest("/api/health", health::router())
        .nest("/api/rpcgreet", rpcgreet::router())
        .nest("/api/gacha/pull_simulation", gacha::router())
        .nest("/api/blog/meta", blog::blog_meta::router())
        .nest("/api/blog/tag", blog::blog_tag::router())
}

// basic handler that responds with a static string
async fn root() -> &'static str {
    "Go to /swagger for docs"
}
