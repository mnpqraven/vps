use super::Service;
use crate::types::BuildKind;
use std::process::Stdio;

/// builds a service
pub fn build_service(service: Service, kind: BuildKind) {
    match kind {
        BuildKind::Script(conf) => build_script(service, conf),
        BuildKind::Docker(conf) => build_docker(service, conf),
        BuildKind::Cargo(conf) => build_cargo(service, conf),
    }
}

fn build_docker(service: Service, _conf: DockerBuildConfig) {
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

fn build_script(_service: Service, _conf: ScriptBuildConfig) {
    todo!()
}

fn build_cargo(service: Service, conf: CargoBuildConfig) {
    let mut args: Vec<String> = ["cargo", "build"].iter().map(|e| e.to_string()).collect();

    if let Some(bin_name) = conf.bin_name {
        let mut bin_arg: Vec<String> = ["--bin", &bin_name].iter().map(|e| e.to_string()).collect();
        args.append(&mut bin_arg);
    }

    let mut cmd = std::process::Command::new("sudo")
        .args(args)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()
        .expect("cargo build error, correct bin name?");

    let status = cmd.wait();
    dbg!(status);
    println!("Service {} built", service.service_name);
}
