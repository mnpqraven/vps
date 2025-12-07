use crate::routes::make_app_router;
use tracing::info;

pub mod routes;
pub mod utils;

pub const API_ADDR: &str = "127.0.0.1:5000";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

    let app_router = make_app_router();

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind(API_ADDR).await.unwrap();

    info!("RUNNING API SERVER @ {API_ADDR}");

    axum::serve(listener, app_router).await?;

    Ok(())
}
