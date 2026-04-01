mod cli;
mod config;

use clap::Parser;
use cli::{Cli, Commands};
use config::{Regret, Timestamp, add_regret, list_regrets, remove_regret};
use std::time::SystemTime;
use tabled::{Table, settings::Style};

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::List => {
            let regrets: Vec<Regret> = list_regrets().unwrap();

            let mut table = Table::new(regrets);
            table.with(Style::modern());

            println!("{}", table);
        }
        Commands::Add(args) => {
            let regret = Regret {
                command: args.command,
                reason: args.reason,
                timestamp: Timestamp(SystemTime::now()),
            };

            add_regret(regret).unwrap();
        }
        Commands::Remove(args) => {
            remove_regret(&args.command).unwrap();
        }
    }
}
