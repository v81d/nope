use clap::{Args, Parser, Subcommand};

/// A CLI tool for keeping track of commands you regret running
#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Manage configuration
    Config(ConfigArgs),
    /// Initialize the program for the specified shell
    Init(InitArgs),
    /// List existing regrets
    List,
    /// Add a new regret
    Add(AddArgs),
    /// Remove an existing regret
    Remove(RemoveArgs),
    /// Check a command against existing regrets
    Check(CheckArgs),
    /// Clear regrets list
    Clear,
}

#[derive(Args)]
pub struct ConfigArgs {
    #[command(subcommand)]
    pub command: ConfigCommands,
}

#[derive(Subcommand)]
pub enum ConfigCommands {
    /// Set similarity threshold for detections
    Threshold(ThresholdArgs),
}

#[derive(Args)]
pub struct ThresholdArgs {
    /// Threshold value between 0.0 and 1.0
    pub value: f64,
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
    pub reason: Option<String>,
}

#[derive(Args)]
pub struct RemoveArgs {
    /// Command ID to remove from regrets list
    pub id: usize,
}

#[derive(Args)]
pub struct CheckArgs {
    /// Command to check against regrets list
    pub command: String,
}
