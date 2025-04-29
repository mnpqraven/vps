pub mod handler;
pub mod routes;
use routes::app_router;
use tracing::info;
use utoipa::OpenApi;
use utoipa_axum::router::OpenApiRouter;
use utoipa_swagger_ui::SwaggerUi;

pub const API_ADDR: &str = "127.0.0.1:5000";

#[derive(OpenApi)]
#[openapi()]
struct ApiDoc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

    let (router, api) = OpenApiRouter::with_openapi(ApiDoc::openapi())
        .merge(app_router())
        .split_for_parts();

    let app_router = router.merge(SwaggerUi::new("/swagger").url("/apidoc/openapi.json", api));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind(API_ADDR).await.unwrap();

    info!("RUNNING API SERVER @ {API_ADDR}");

    axum::serve(listener, app_router).await?;

    Ok(())
}
