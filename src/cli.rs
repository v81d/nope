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
    List,
    Add(AddArgs),
    Remove(RemoveArgs),
}

#[derive(Args)]
pub struct AddArgs {
    /// Command to add to regrets list
    pub command: String,

    /// Brief description of regret
    pub reason: String,
}

#[derive(Args)]
pub struct RemoveArgs {
    /// Command to remove from regrets list
    pub command: String,
}
