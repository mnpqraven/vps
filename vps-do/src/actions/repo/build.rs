use std::{
    io::Write,
    path::Path,
    process::{Command, Stdio},
};

use crate::{
    actions::repo_list::{repo_list, BuildKind, CargoBuildConfig, Service},
    utils::config_parse::read_config_toml,
};

pub fn build_all() {
    println!("building all services");
    let list = repo_list();
    for service in list {
        match &service.build {
            BuildKind::Script(_script_build_config) => {},
            BuildKind::Docker(_docker_build_config) => {},
            BuildKind::Cargo(config) => build_cargo(&service, config),
        }
    }
}

fn build_cargo(service: &Service, config: &CargoBuildConfig) {
    // this relative root won't cd to the correct path right away, need to
    // join with home path from conf toml
    let default_pwd = std::env::current_dir().unwrap();
    let Service { relative_root, .. } = service;
    let conf = read_config_toml().unwrap();
    let true_path = Path::new(&conf.general.home).join(relative_root);
    let true_path = true_path.to_string_lossy();

    let mut args = vec!["build"];
    if let Some(bin_name) = &config.bin_name {
        args.push("--bin");
        args.push(bin_name.as_str());
    }
    dbg!(&true_path);

    // cd
    let _ = std::env::set_current_dir(&*true_path);
    // cargo build
    let mut cmd = Command::new("cargo")
        .args(args)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()
        // TODO: unwrap
        .expect("build failed, maybe clone the repos first ?");

    cmd.wait().unwrap();
    let _ = std::env::set_current_dir(default_pwd);
}
