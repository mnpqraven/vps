use crate::utils::{args::ServiceCommands, read_config_toml};
use list::list_running_service;
use service::{service_action_server::ServiceAction, Service, ServiceListResponse};
use std::path::Path;
use tonic::{Request, Response, Status};
use types::{
    build::{BuildConfig, BuildKind, CargoBuildConfig, DockerBuildConfig},
    deployment::DeploymentKind,
};

pub mod build;
pub mod deploy;
pub mod list;

pub mod service {
    tonic::include_proto!("service");
}

pub mod types {
    pub mod build {
        tonic::include_proto!("build");
    }
    pub mod deployment {
        tonic::include_proto!("deployment");
    }
}

pub(crate) const SERVICE_DESCRIPTOR_SET: &[u8] =
    tonic::include_file_descriptor_set!("service_descriptor");

#[derive(Debug, Default)]
struct Sv {}

#[tonic::async_trait]
impl ServiceAction for Sv {
    async fn list(&self, _: Request<()>) -> Result<Response<ServiceListResponse>, Status> {
        let services = repo_list();
        Ok(Response::new(ServiceListResponse { services }))
    }
}

impl service::Service {
    pub fn absolute_path(&self) -> String {
        let conf = read_config_toml().unwrap();
        let true_path = Path::new(&conf.general.home).join(self.relative_root.clone());
        let true_path = true_path.to_string_lossy();
        true_path.to_string()
    }
}

pub fn handle_service_arg(arg: &ServiceCommands) {
    match arg {
        ServiceCommands::List => list_running_service(),
        ServiceCommands::Deploy => todo!(),
    }
}

/// returns a list of service repositories
/// TODO: JSON somewhere else
pub fn repo_list() -> Vec<Service> {
    let vps_rpc: Service = Service {
        service_name: "vps-rpc".to_string(),
        url: "https://github.com/mnpqraven/vps".to_string(),
        relative_root: "service_repos/vps".to_string(),
        path: None,
        deployment: Some(DeploymentKind::Shell.into()),
        build_config: Some(BuildConfig {
            kind: BuildKind::Cargo.into(),
            script_config: None,
            docker_config: None,
            cargo_config: Some(CargoBuildConfig {
                bin_name: Some("vps-rpc".to_string()),
            }),
        }),
    };

    let vps_api: Service = Service {
        service_name: "vps-api".to_string(),
        url: "https://github.com/mnpqraven/vps".to_string(),
        relative_root: "service_repos/vps".to_string(),
        path: None,
        deployment: Some(DeploymentKind::Shell.into()),
        build_config: Some(BuildConfig {
            kind: BuildKind::Cargo.into(),
            script_config: None,
            docker_config: None,
            cargo_config: Some(CargoBuildConfig {
                bin_name: Some("vps-api".to_string()),
            }),
        }),
    };

    let vps_cli: Service = Service {
        service_name: "vps-api".to_string(),
        url: "https://github.com/mnpqraven/vps".to_string(),
        relative_root: "service_repos/vps".to_string(),
        path: None,
        deployment: None,
        build_config: Some(BuildConfig {
            kind: BuildKind::Cargo.into(),
            cargo_config: Some(CargoBuildConfig {
                bin_name: Some("vps-do".to_string()),
            }),
            script_config: None,
            docker_config: None,
        }),
    };

    let othi_blog: Service = Service {
        service_name: "othi".to_string(),
        url: "https://github.com/mnpqraven/othi-monorepo".to_string(),
        relative_root: "service_repos/othi-monorepo".to_string(),
        path: None,
        deployment: Some(DeploymentKind::Docker.into()),
        build_config: Some(BuildConfig {
            kind: BuildKind::Docker.into(),
            script_config: None,
            docker_config: Some(DockerBuildConfig { is_compose: true }),
            cargo_config: None,
        }),
    };

    let repos = vec![vps_rpc, vps_api, vps_cli, othi_blog];
    repos
}
