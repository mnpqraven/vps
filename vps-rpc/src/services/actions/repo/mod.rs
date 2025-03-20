use crate::rpc::service::{
    repo_action_server::RepoAction, BuildRequest, BuildResponse, RepoListResponse,
};
use build::handle_build;
use tonic::{Request, Response, Status};

use super::service::repo_list;

pub mod build;
pub mod clone;
pub mod pull;

#[derive(Debug, Default)]
pub struct RepoRpc {}

#[tonic::async_trait]
impl RepoAction for RepoRpc {
    async fn list(&self, _: Request<()>) -> Result<Response<RepoListResponse>, Status> {
        let services = repo_list();
        Ok(Response::new(RepoListResponse { repos: services }))
    }

    async fn build(&self, req: Request<BuildRequest>) -> Result<Response<BuildResponse>, Status> {
        let req = req.into_inner();

        let services = repo_list();
        if req.service_names.is_empty() {
            println!("build all (WIP");
            for service in services {
                handle_build(&service);
            }

            return Ok(Response::new(BuildResponse {
                status: "WIP".to_string(),
            }));
        }

        for maybe_service in &req.service_names {
            let find = services.iter().find(|e| e.service_name == *maybe_service);
            if find.is_none() {
                panic!("service not found")
            }
        }

        services
            .iter()
            .filter(|e| req.service_names.contains(&e.service_name))
            .for_each(handle_build);

        Ok(Response::new(BuildResponse {
            status: "WIP".to_string(),
        }))
    }
}
