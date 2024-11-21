use crate::{actions::repo_list::repo_list, utils::config_parse::read_config_toml};
use std::{
    collections::HashSet,
    error::Error,
    fs,
    path::Path,
    process::{Command, Stdio},
};

/// clone all repos
pub fn clone_all() {
    let list = repo_list();
    let unique_urls: Vec<String> = list
        .into_iter()
        .map(|e| e.url)
        .collect::<HashSet<_>>()
        .into_iter()
        .collect();

    // TODO: check for success/failed tasks
    for url in &unique_urls {
        clone_single(url).unwrap();
    }
    println!("Cloned {} repos", &unique_urls.len());
}

/// clones a service from github into path
/// TODO: privatize
pub fn clone_single(url: &str) -> Result<(), Box<dyn Error>> {
    let config = read_config_toml()?;
    let out_dir = Path::new(&config.general.home).join("service_repos");
    // create if not exist
    // TODO: UNWRAP
    fs::create_dir_all(out_dir.clone()).unwrap();

    // git -C <path> clone <url>
    let mut cmd = Command::new("git")
        .args(["-C", &out_dir.to_string_lossy(), "clone", url])
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()
        .unwrap();

    cmd.wait().unwrap();
    Ok(())
}
