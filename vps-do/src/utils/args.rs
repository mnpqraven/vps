use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct CliArgs {
    #[command(subcommand)]
    pub command: ActionCategory,
}

#[derive(Subcommand)]
pub enum ActionCategory {
    /// repository actions (list, clone, pull, redeploy)
    #[command(subcommand)]
    Repo(RepoCommands),
    #[command(subcommand)]
    Service(ServiceCommands),
}

#[derive(Subcommand, Debug)]
pub enum RepoCommands {
    List,
    Clone,
    Pull,
}

#[derive(Subcommand, Debug)]
pub enum ServiceCommands {
    /// Lists running services
    List,
    Build,
    Deploy,
}
