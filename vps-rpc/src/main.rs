use tonic::transport::Server;
use vps_rpc::{
    layer::{cors, grpc_web},
    // rpc::service::tag_action_server::TagActionServer,
    services::{
        database::blog_tag::BlogTagRpc,
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

    let conn = database::get_db().await?;

    // description service for web ui completion
    let descriptor_service = tonic_reflection::server::Builder::configure()
        .register_encoded_file_descriptor_set(DESCRIPTOR_SET)
        .build_v1alpha()?;

    tracing::info!("RUNNING gRPC SERVER @ {RPC_ADDR}");

    Server::builder()
        .accept_http1(true)
        .layer(cors())
        .layer(grpc_web())
        .trace_fn(|_| tracing::debug_span!("rpc"))
        .add_service(descriptor_service)
        .add_service(GreeterServer::new(GreeterRpc::default()))
        // .add_service(TagActionServer::new(TagRpc { conn }))
        .serve(RPC_ADDR.parse()?)
        .await?;

    Ok(())
}
