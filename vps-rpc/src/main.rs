use proto_types::{
    blog::{
        meta::blog_meta_service_server::BlogMetaServiceServer,
        tag::blog_tag_service_server::BlogTagServiceServer,
    },
    greeter_server::GreeterServer,
    DESCRIPTOR_SET,
};
use services::{
    database::{blog::meta::BlogMetaRpc, blog_tag::BlogTagRpc},
    greeter::GreeterRpc,
};
use tonic::transport::Server;
use vps_rpc::{
    layer::{cors, grpc_web},
    RPC_ADDR,
};

pub mod services;
pub mod utils;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    let db = database::get_db().await?;

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
        .add_service(BlogTagServiceServer::new(BlogTagRpc { conn: db.clone() }))
        .add_service(BlogMetaServiceServer::new(BlogMetaRpc { conn: db.clone() }))
        .serve(RPC_ADDR.parse()?)
        .await?;

    Ok(())
}
