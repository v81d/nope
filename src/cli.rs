use clap::{Args, Parser, Subcommand};

/// A CLI tool for keeping track of commands you regret running.
#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Configure detection settings
    Config(ConfigArgs),
    /// Initialize the program for a given shell
    Init(InitArgs),
    /// List existing regrets
    List,
    /// Add a new regret
    Add(AddArgs),
    /// Remove an existing regret
    Remove(RemoveArgs),
    /// Check a command against existing regrets
    Check(CheckArgs),
    /// Clear the regrets list
    Clear,
}

#[derive(Args)]
pub struct ConfigArgs {
    #[command(subcommand)]
    pub command: ConfigCommands,
}

#[derive(Subcommand)]
pub enum ConfigCommands {
    /// Enable the command hook and show an alert for detections
    Enable,
    /// Disable the command hook and ignore detections
    Disable,
    /// Set the similarity threshold for detections (between 0 and 1)
    Threshold(ThresholdArgs),
}

#[derive(Args)]
pub struct ThresholdArgs {
    /// The similarity threshold above which an input should trigger an alert.
    pub value: f64,
}

#[derive(Args)]
pub struct InitArgs {
    /// The shell to initialize the program for
    pub shell: String,
}

#[derive(Args)]
pub struct AddArgs {
    /// The command to add to regrets list
    pub command: String,
    /// The reason for the regret
    pub reason: Option<String>,
}

#[derive(Args)]
pub struct RemoveArgs {
    /// The command ID to remove from the regrets list
    pub id: usize,
}

#[derive(Args)]
pub struct CheckArgs {
    /// The command to check against the regrets list
    pub command: Option<String>,
}
