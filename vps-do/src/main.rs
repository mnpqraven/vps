use actions::{repo::handle_repo_arg, service::handle_service_arg};
use clap::Parser;
use utils::args::{ActionCategory, CliArgs};

mod actions;
mod utils;

fn main() {
    let cli = CliArgs::parse();

    match &cli.command {
        ActionCategory::Repo(repo_arg) => handle_repo_arg(repo_arg),
        ActionCategory::Service(service_arg) => handle_service_arg(service_arg),
    }
}
