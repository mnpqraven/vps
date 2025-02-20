pub mod routes;
pub mod handler;
use routes::app_router;

pub const API_ADDR: &str = "[::1]:4000";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

    let app = app_router();

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind(API_ADDR).await.unwrap();

    axum::serve(listener, app).await?;

    Ok(())
}
