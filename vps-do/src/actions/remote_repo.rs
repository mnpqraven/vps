use std::{fs, process::Command};

/// action to take with remote repo
enum FsResolveAction {
    Clone,
    Pull,
    Noop,
}

/// clones a service from github into path
/// TODO: privatize
pub fn clone_single(url: &str) {
    let out_dir = "./service_repos";
    // create if not exist
    // TODO: UNWRAP
    fs::create_dir_all(out_dir).unwrap();

    // git -C <path> clone <url>
    let _clone = Command::new("git")
        .args(["-C", out_dir, "clone", url])
        .output()
        .unwrap();
    println!("stdout: {}", String::from_utf8_lossy(&_clone.stdout));
    println!("stderr: {}", String::from_utf8_lossy(&_clone.stderr));
}

/// pull a service repo in path
pub fn pull_single(name: &str) {
    let out_dir = format!("/home/ubuntu/service_repos/{name}");
    let _clone = Command::new("git")
        .args(["-C", &out_dir, "pull"])
        .output()
        .unwrap();
}

/// deploy/build the repo
fn deploy() {}
