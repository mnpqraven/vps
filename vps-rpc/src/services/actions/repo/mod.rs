use super::service::repo_list;
use crate::utils::args::{RepoBuildTarget, RepoCommands};
use build::build_all;
use clone::clone_all;
use pull::pull_single;
use std::collections::HashSet;

pub mod build;
pub mod clone;
pub mod pull;

pub fn handle_repo_arg(arg: &RepoCommands) {
    let list = repo_list();
    match arg {
        RepoCommands::List => {
            let list = list
                .iter()
                .map(|e| e.service_name.to_owned())
                .collect::<Vec<String>>();
            dbg!(list);
        }
        RepoCommands::Clone => clone_all(),
        RepoCommands::Pull => {
            // TODO: unique urls
            let unique_paths: Vec<String> = list
                .into_iter()
                .map(|e| e.relative_root)
                .collect::<HashSet<_>>()
                .into_iter()
                .collect();
            for path in unique_paths {
                pull_single(&path);
            }
            println!("Pulling done");
        }
        RepoCommands::Build(e) => handle_build(e),
    }
}

fn handle_build(args: &RepoBuildTarget) {
    println!("build_all (WIP)");
    dbg!(args);
    match args {
        RepoBuildTarget::All => build_all(),
        RepoBuildTarget::Bins { bins } => {
            // TODO: check existence
            println!("building {} service(s): {}", bins.len(), bins.join(", "));
        }
    }
}
