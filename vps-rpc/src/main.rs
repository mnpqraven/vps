use http::Method;
use tonic::transport::Server;
use tower_http::cors::Any;
use vps_rpc::{
    rpc::service::tag_action_server::TagActionServer,
    services::{
        database::blog_tag::TagRpc,
        greeter::{greeter_server::GreeterServer, GreeterRpc},
    },
    RPC_ADDR,
};

pub mod rpc;
pub mod services;
pub mod utils;

const DESCRIPTOR_SET: &[u8] = tonic::include_file_descriptor_set!("descriptor");

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    // description service for web ui completion
    let descriptor_service = tonic_reflection::server::Builder::configure()
        .register_encoded_file_descriptor_set(DESCRIPTOR_SET)
        .build_v1alpha()?;

    tracing::info!("RUNNING gRPC SERVER @ {RPC_ADDR}");

    // @ref https://connectrpc.com/docs/cors#configurations-by-protocol
    let cors_layer = tower_http::cors::CorsLayer::new()
        .allow_methods([Method::GET, Method::POST])
        .allow_headers(Any)
        .allow_origin([
            // local web ui
            "http://127.0.0.1:4000".parse().unwrap(),
            "http://localhost:4000".parse().unwrap(),
            // prod origins
            // "https://othi.dev".parse().unwrap(),
        ]);
    let grpc_web_layer = tonic_web::GrpcWebLayer::new();

    Server::builder()
        .accept_http1(true)
        .layer(cors_layer)
        .layer(grpc_web_layer)
        .trace_fn(|_| tracing::debug_span!("rpc"))
        .add_service(descriptor_service)
        .add_service(GreeterServer::new(GreeterRpc::default()))
        .add_service(TagActionServer::new(TagRpc::default()))
        .serve(RPC_ADDR.parse()?)
        .await?;

    Ok(())
}
