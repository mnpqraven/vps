use actions::{
    remote_repo::{clone_single, pull_single},
    repo_list::{repo_list, Service},
    service::handle_service_arg,
};
use clap::Parser;
use utils::args::{ActionCategory, CliArgs, RepoCommands};

mod actions;
mod utils;

fn main() {
    let cli = CliArgs::parse();

    // You can check for the existence of subcommands, and if found use their
    // matches just as you would the top level cmd
    let list = repo_list();
    match &cli.command {
        ActionCategory::Repo(RepoCommands::List) => {
            let list = list
                .iter()
                .map(|e| e.service_name.to_owned())
                .collect::<Vec<String>>();
            dbg!(list);
        }
        ActionCategory::Repo(RepoCommands::Clone) => {
            // for now clone
            // TODO: parent fn handling deciding between clone or pull
            for Service { url, .. } in list {
                clone_single(&url);
            }
            println!("cloning done");
        }
        ActionCategory::Repo(RepoCommands::Pull) => {
            for Service { url, .. } in list {
                pull_single(&url);
            }
            println!("pulling done");
        }
        ActionCategory::Service(service_arg) => handle_service_arg(service_arg),
    }

    // Continued program logic goes here...
}
