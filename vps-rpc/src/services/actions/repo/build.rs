use crate::services::actions::service::{repo_list, BuildKind, Service};
use std::{
    env,
    process::{Command, Stdio},
};

pub fn build_all() {
    println!("building all services");
    let list = repo_list();
    for service in list {
        match &service.build_config {
            BuildKind::Script => {}
            BuildKind::Docker => build_docker(&service),
            BuildKind::Cargo => build_cargo(&service),
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
    if let Some(bin_name) = &service.build_config.unwrap().bin_name {
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

    let args = match config.is_compose {
        true => vec!["docker", "compose", "build", "--no-cache"],
        // TODO: test
        false => vec!["docker", "build"],
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
