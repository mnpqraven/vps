use super::service::{BuildKind, CargoBuildConfig, DeploymentKind, DockerBuildConfig, Service};

/// returns a list of service repositories
/// TODO: JSON somewhere else
pub fn repo_list() -> Vec<Service> {
    let vps_rpc: Service = Service {
        service_name: "vps-rpc".to_string(),
        url: "https://github.com/mnpqraven/vps".to_string(),
        relative_root: "service_repos/vps".to_string(),
        _path: None,
        build: BuildKind::Cargo(CargoBuildConfig {
            bin_name: Some("vps-rpc".to_string()),
        }),
        // TODO:
        _deployment: Some(DeploymentKind::Shell),
    };

    let vps_api: Service = Service {
        service_name: "vps-api".to_string(),
        url: "https://github.com/mnpqraven/vps".to_string(),
        relative_root: "service_repos/vps".to_string(),
        _path: None,
        build: BuildKind::Cargo(CargoBuildConfig {
            bin_name: Some("vps-api".to_string()),
        }),
        // TODO:
        _deployment: Some(DeploymentKind::Shell),
    };

    let vps_cli: Service = Service {
        service_name: "vps-api".to_string(),
        url: "https://github.com/mnpqraven/vps".to_string(),
        relative_root: "service_repos/vps".to_string(),
        _path: None,
        build: BuildKind::Cargo(CargoBuildConfig {
            bin_name: Some("vps-do".to_string()),
        }),
        _deployment: None,
    };

    let othi_blog: Service = Service {
        service_name: "othi".to_string(),
        url: "https://github.com/mnpqraven/othi-monorepo".to_string(),
        relative_root: "service_repos/othi-monorepo".to_string(),
        _path: None,
        build: BuildKind::Docker(DockerBuildConfig { is_compose: true }),
        _deployment: Some(DeploymentKind::Docker),
    };

    let repos = vec![vps_rpc, vps_api, vps_cli, othi_blog];
    repos
}
