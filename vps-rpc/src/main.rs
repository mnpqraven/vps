use rpc::service::service_action_server::ServiceActionServer;
use services::{
    actions::service::ServiceRpc,
    greeter::{greeter_server::GreeterServer, MyGreeter},
};
use tonic::transport::Server;
use tracing::info;
use vps_rpc::RPC_ADDR;

pub mod rpc;
pub mod services;
pub mod utils;

const DESCRIPTOR_SET: &[u8] = tonic::include_file_descriptor_set!("descriptor");

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

    let descriptor_service = tonic_reflection::server::Builder::configure()
        .register_encoded_file_descriptor_set(DESCRIPTOR_SET)
        .build_v1alpha()?;

    info!("RUNNING gRPC SERVER @ {RPC_ADDR}");

    Server::builder()
        .add_service(descriptor_service)
        .add_service(GreeterServer::new(MyGreeter::default()))
        .add_service(ServiceActionServer::new(ServiceRpc::default()))
        .serve(RPC_ADDR.parse()?)
        .await?;

    Ok(())
}
