pub mod routes;
pub mod handler;
use routes::app_router;
use tracing::info;

pub const API_ADDR: &str = "127.0.0.1:4000";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

    let app = app_router();

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind(API_ADDR).await.unwrap();
    info!("{} API SERVER UP", API_ADDR);

    axum::serve(listener, app).await?;

    Ok(())
}
