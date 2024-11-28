use rpc::{
    service::service_action_server::ServiceActionServer, REPO_DESCRIPTOR_SET,
    SERVICE_DESCRIPTOR_SET,
};
use services::{
    actions::service::ServiceRpc,
    greeter::{greeter_server::GreeterServer, MyGreeter, HELLO_DESCRIPTOR_SET},
};
use tonic::transport::Server;

pub mod rpc;
pub mod services;
pub mod utils;

pub const RPC_ADDR: &str = "[::1]:4001";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let descriptor_service = tonic_reflection::server::Builder::configure()
        .register_encoded_file_descriptor_set(HELLO_DESCRIPTOR_SET)
        .register_encoded_file_descriptor_set(REPO_DESCRIPTOR_SET)
        .register_encoded_file_descriptor_set(SERVICE_DESCRIPTOR_SET)
        .build_v1alpha()?;

    println!("spinning up gRPC server ...");

    Server::builder()
        .add_service(descriptor_service)
        .add_service(GreeterServer::new(MyGreeter::default()))
        .add_service(ServiceActionServer::new(ServiceRpc::default()))
        .serve(RPC_ADDR.parse()?)
        .await?;

    Ok(())
}
