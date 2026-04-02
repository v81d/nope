use clap::{Args, Parser, Subcommand};

///  A CLI tool for keeping track of commands you regret running.
#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Init(InitArgs),
    List,
    Add(AddArgs),
    Remove(RemoveArgs),
    Check(CheckArgs),
}

#[derive(Args)]
pub struct InitArgs {
    /// The shell to initialize
    pub shell: String,
}

#[derive(Args)]
pub struct AddArgs {
    /// Command to add to regrets list
    pub command: String,

    /// Brief reason for regret
    pub reason: String,
}

#[derive(Args)]
pub struct RemoveArgs {
    /// Command to remove from regrets list
    pub command: String,
}

#[derive(Args)]
pub struct CheckArgs {
    /// Command to check against regrets list
    pub command: String,
}
