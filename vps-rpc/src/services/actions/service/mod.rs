use crate::utils::{args::ServiceCommands, read_config_toml};
use list::list_running_service;
use std::path::Path;

pub mod build;
pub mod deploy;
pub mod list;

// TODO: json schema after making builder
pub struct Service {
    pub service_name: String,
    pub url: String,
    /// optional path argument when the build directory is not the repo's root
    pub _path: Option<String>,
    /// if the service can be deployed then the method is specified here
    pub _deployment: Option<DeploymentKind>,
    pub build: BuildKind,
    /// relative path (from $HOME) to the repo directory
    pub relative_root: String,
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

impl Service {
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
