use std::process::Stdio;

use crate::actions::repo_list::{BuildKind, Service};

/// builds a service
pub fn build_service(service: Service, kind: BuildKind) {
    match kind {
        BuildKind::Script => build_script(service),
        BuildKind::Docker => build_docker(service),
        BuildKind::Cargo => build_cargo(service),
    }
}

fn build_docker(service: Service) {
    let mut cmd = std::process::Command::new("sudo")
        .args([
            "docker",
            "compose",
            "build",
            "--no-cache",
            &service.service_name,
        ])
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()
        .expect("compose build error, correct service name?");

    let status = cmd.wait();
    dbg!(status);
}

fn build_script(service: Service) {}
fn build_cargo(service: Service) {}
