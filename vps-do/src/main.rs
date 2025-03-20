use clap::Parser;
use cli::repo::handle_repo_arg;
use data_shapes::{ActionCategory, CliArgs};

mod cli;
mod utils;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = CliArgs::parse();

    match &cli.command {
        ActionCategory::Repo(repo_arg) => handle_repo_arg(repo_arg).await,
        ActionCategory::Service(service_arg) => {
            // handle_service_arg(service_arg)
        }
    }

    Ok(())
}
