use crate::routes::make_app_router;
use tower_http::cors::{Any, CorsLayer};
use tracing::info;

pub mod routes;
pub mod utils;

pub const API_ADDR: &str = "127.0.0.1:5000";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

    let cors = CorsLayer::new()
        .allow_methods(Any)
        .allow_headers(Any)
        .allow_origin([
            // local web ui
            "http://127.0.0.1:3000".parse().unwrap(),
            "http://127.0.0.1:4004".parse().unwrap(),
            "http://localhost:3000".parse().unwrap(),
            "http://localhost:4004".parse().unwrap(),
            // prod origins
        ]);
    let app_router = make_app_router().layer(cors);

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind(API_ADDR).await.unwrap();

    info!("RUNNING API SERVER @ {API_ADDR}");

    axum::serve(listener, app_router).await?;

    Ok(())
}
