use std::path::Path;

use crate::utils::config_parse::read_config_toml;

// TODO: json schema after making builder
pub struct Service {
    pub service_name: String,
    pub url: String,
    /// optional path argument when the build directory is not the repo's root
    _path: Option<String>,
    /// if the service can be deployed then the method is specified here
    _deployment: Option<DeploymentKind>,
    pub build: BuildKind,
    /// relative path (from $HOME) to the repo directory
    pub relative_root: String,
}

impl Service {
    pub fn absolute_path(&self) -> String {
        let conf = read_config_toml().unwrap();
        let true_path = Path::new(&conf.general.home).join(self.relative_root.clone());
        let true_path = true_path.to_string_lossy();
        true_path.to_string()
    }
}

pub enum DeploymentKind {
    /// if the deployment is containerized
    Docker,
    Shell,
}

pub enum BuildKind {
    Script(ScriptBuildConfig),
    Docker(DockerBuildConfig),
    Cargo(CargoBuildConfig),
}

pub struct ScriptBuildConfig {}
pub struct DockerBuildConfig {
    pub is_compose: bool,
}
pub struct CargoBuildConfig {
    /// if the build target is not default and needs to be built via `--bin`
    /// then this is the bin name
    pub bin_name: Option<String>,
}

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
