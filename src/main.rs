mod checker;
mod cli;
mod config;
mod init;

use checker::check_command;
use clap::Parser;
use cli::{Cli, Commands};
use colored::Colorize;
use config::{Reason, Regret, Timestamp, add_regret, list_regrets, remove_regret};
use init::initialize_shell;
use std::time::SystemTime;
use tabled::{Table, Tabled, settings::Style};

#[derive(Tabled)]
struct RegretListRow {
    #[tabled(rename = "ID")]
    id: usize,
    #[tabled(rename = "Command")]
    command: String,
    #[tabled(rename = "Reason")]
    reason: String,
    #[tabled(rename = "Timestamp")]
    timestamp: String,
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Init(args) => {
            initialize_shell(&args.shell);
        }
        Commands::List => {
            let regrets = list_regrets()
                .unwrap()
                .into_iter()
                .enumerate()
                .map(|(i, r)| RegretListRow {
                    id: i,
                    command: r.command,
                    reason: r.reason.get(),
                    timestamp: r.timestamp.to_string(),
                });

            let mut table = Table::new(regrets);
            table.with(Style::modern());

            println!("{}", table);
        }
        Commands::Add(args) => {
            let regret = Regret {
                command: args.command,
                reason: Reason(args.reason),
                timestamp: Timestamp(SystemTime::now()),
            };

            add_regret(regret).unwrap();
        }
        Commands::Remove(args) => {
            remove_regret(args.id).unwrap();
        }
        Commands::Check(args) => {
            if let Some(regret) = check_command(&args.command) {
                eprintln!("{}", "This command is in your regret list.".bold().red());
                eprintln!("{} {}", "Reason:".red(), regret.reason.get().yellow());
                std::process::exit(1);
            }
        }
    }
}
