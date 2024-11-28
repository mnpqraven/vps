use crate::rpc::{service::Service, types::deployment::DeploymentKind};
use std::process::Stdio;

pub fn deploy_service(service: Service, kind: DeploymentKind) {
    match kind {
        DeploymentKind::Docker => deploy_docker(service),
        DeploymentKind::Shell => deploy_shell(service),
    }
}

fn deploy_docker(service: Service) {
    let mut cmd = std::process::Command::new("sudo")
        .args(["docker", "compose", "up", "--detach", &service.service_name])
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()
        .expect("compose deploy error, correct service name?");

    let status = cmd.wait().unwrap();
    dbg!(status);
}

fn deploy_shell(_service: Service) {
    println!("WIP deploy shell")
}
