use proto_types::service::{HealthResponse, health_service_server::HealthService};
use tonic::Request;

use crate::utils::TonicResult;

#[derive(Debug, Default)]
pub struct HealthRpc;

#[tonic::async_trait]
impl HealthService for HealthRpc {
    async fn health(&self, _request: Request<()>) -> TonicResult<HealthResponse> {
        Ok(HealthResponse {
            response: "ok from rpc server".into(),
        }
        .into())
    }
}
