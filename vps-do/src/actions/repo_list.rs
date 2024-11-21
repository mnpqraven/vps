// TODO: json schema after making builder
pub struct Service {
    pub service_name: String,
    pub url: String,
    /// optional path argument when the build directory is not the repo's root
    pub path: Option<String>,
    /// if the service can be deployed then the method is specified here
    pub deployment: Option<DeploymentKind>,
    pub build: BuildKind,
    /// relative path (from $HOME) to the repo directory
    pub relative_root: String,
}

pub enum DeploymentKind {
    Docker,
}

pub enum BuildKind {
    Script,
    Docker,
    Cargo,
}

/// returns a list of service repositories
/// TODO: JSON somewhere else
pub fn repo_list() -> Vec<Service> {
    let vps_center_hub: Service = Service {
        service_name: "vps-center-hub".to_string(),
        url: "https://github.com/mnpqraven/vps-center-hub".to_string(),
        relative_root: "service_repos/vps-center-hub".to_string(),
        path: None,
        build: BuildKind::Cargo,
        deployment: None,
    };

    let vps_center_api: Service = Service {
        service_name: "vps-center-api".to_string(),
        url: "https://github.com/mnpqraven/vps-center-api".to_string(),
        relative_root: "service_repos/vps-center-api".to_string(),
        path: None,
        build: BuildKind::Cargo,
        deployment: None,
    };

    let othi_blog: Service = Service {
        service_name: "othi".to_string(),
        url: "https://github.com/mnpqraven/othi-monorepo".to_string(),
        relative_root: "service_repos/othi-monorepo".to_string(),
        path: None,
        build: BuildKind::Docker,
        deployment: Some(DeploymentKind::Docker),
    };

    let repos = vec![vps_center_hub, vps_center_api, othi_blog];
    repos
}
