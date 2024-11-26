use super::{DeploymentKind, Service};

pub fn deploy_service(service: Service, kind: DeploymentKind) {
    match kind {
        DeploymentKind::Docker => deploy_docker(service),
        DeploymentKind::Shell => deploy_shell(service),
    }
}

fn deploy_docker(service: Service) {
    let cmd = std::process::Command::new("sudo")
        .args(["docker", "compose", "up", "--detach", &service.service_name])
        .output()
        .expect("compose deploy error, correct service name?");
}

fn deploy_shell(service: Service) {
    println!("WIP deploy shell")
}
