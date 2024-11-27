use super::{types::build::BuildKind, Service};
use std::process::Stdio;

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

fn build_script(_service: Service) {
    todo!()
}

fn build_cargo(service: Service) {
    let mut args: Vec<String> = ["cargo", "build"].iter().map(|e| e.to_string()).collect();

    if let Some(bin_name) = service
        .build_config
        .map(|e| e.cargo_config.map(|f| f.bin_name))
        .flatten()
        .flatten()
    {
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
