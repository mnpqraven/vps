use crate::{
    rpc::{service::Service, types::build::BuildKind},
    services::actions::service::repo_list,
};
use std::{
    env,
    process::{Command, Stdio},
};

pub(super) fn handle_build(service: &Service) {
    match service.build_config.as_ref().map(|e| e.kind()) {
        Some(BuildKind::Cargo) => build_cargo(service),
        Some(BuildKind::Docker) => build_docker(service),
        Some(BuildKind::Script) => build_script(service),
        None => {
            println!("Build config not specified, nothing will be built");
        }
    }
}

pub fn build_all() {
    println!("building all services");
    let list = repo_list();
    for service in list {
        match &service.build_config.as_ref().map(|e| e.kind()) {
            Some(BuildKind::Script) => {}
            Some(BuildKind::Docker) => build_docker(&service),
            Some(BuildKind::Cargo) => build_cargo(&service),
            _ => {}
        }
    }
}

fn build_cargo(service: &Service) {
    let absolute_path = service.absolute_path();
    dbg!(&absolute_path);
    // this relative root won't cd to the correct path right away, need to
    // join with home path from conf toml
    let default_pwd = env::current_dir().unwrap();
    // cd
    let _ = env::set_current_dir(absolute_path);

    // either "build" or "build --bin [name]"
    let mut args = vec!["build"];
    if let Some(bin_name) = &service
        .build_config
        .as_ref()
        // TODO: unwrap
        .unwrap()
        .cargo_config
        .as_ref()
        // TODO: unwrap
        .unwrap()
        .bin_name
    {
        args.push("--bin");
        args.push(bin_name.as_str());
    }
    // cargo build
    let mut cmd = Command::new("cargo")
        .args(args)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()
        // TODO: unwrap
        .expect("build failed, maybe clone the repos first ?");

    cmd.wait().unwrap();
    let _ = env::set_current_dir(default_pwd);
}

fn build_docker(service: &Service) {
    let absolute_path = service.absolute_path();
    let default_pwd = env::current_dir().unwrap();
    // cd
    let _ = env::set_current_dir(absolute_path);

    let args = match service
        .build_config
        .as_ref()
        .and_then(|e| e.docker_config.map(|f| f.is_compose))
    {
        Some(true) => vec!["docker", "compose", "build", "--no-cache"],
        // TODO: test
        _ => vec!["docker", "build"],
    };
    let mut cmd = Command::new("sudo")
        .args(args.as_slice())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()
        // TODO: unwrap
        .expect("build failed, maybe clone the repos first ?");

    cmd.wait().unwrap();
    let _ = env::set_current_dir(default_pwd);
}

fn build_script(_service: &Service) {}
