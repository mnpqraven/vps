use std::{
    path::Path,
    process::{Command, Stdio},
};

use crate::utils::config_parse::read_config_toml;

/// pull a service repo in path
pub fn pull_single(path: &str) {
    println!("Pulling {path}");
    let config = read_config_toml().unwrap();
    let out_dir = Path::new(&config.general.home).join(path);

    let mut cmd = Command::new("git")
        .args(["-C", &out_dir.to_string_lossy(), "pull"])
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()
        .unwrap();
    cmd.wait().unwrap();
}
