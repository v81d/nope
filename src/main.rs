mod cli;
mod config;

use clap::Parser;
use cli::{Cli, Commands};
use config::{Regret, add_regret, remove_regret};
use std::time::SystemTime;

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Add(args) => {
            let regret = Regret {
                command: args.command,
                reason: args.reason,
                timestamp: SystemTime::now(),
            };

            add_regret(regret).unwrap();
        }
        Commands::Remove(args) => {
            remove_regret(&args.command).unwrap();
        }
        _ => (),
    }
}
