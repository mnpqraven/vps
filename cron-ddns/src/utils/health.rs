use proto_types::{
    TonicResult,
    service::{HealthResponse, health_service_server::HealthService},
};
use tonic::Request;

#[derive(Debug, Default)]
pub struct CronHealthRpc;

#[tonic::async_trait]
impl HealthService for CronHealthRpc {
    async fn health(&self, _request: Request<()>) -> TonicResult<HealthResponse> {
        Ok(HealthResponse {
            response: "ok from cron rpc".into(),
        }
        .into())
    }
}
