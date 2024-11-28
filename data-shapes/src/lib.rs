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
    #[command(subcommand)]
    Build(RepoBuildTarget),
}

#[derive(Subcommand, Debug)]
pub enum RepoBuildTarget {
    All,
    Bins { bins: Vec<String> },
}

#[derive(Subcommand, Debug)]
pub enum ServiceCommands {
    /// Lists running services
    List,
    Deploy,
}
