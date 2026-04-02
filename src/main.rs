mod checker;
mod cli;
mod config;
mod init;

use checker::check_command;
use clap::Parser;
use cli::{Cli, Commands};
use colored::Colorize;
use config::{Regret, Timestamp, add_regret, list_regrets, remove_regret};
use init::initialize_shell;
use std::io::Write;
use std::time::SystemTime;
use tabled::{Table, settings::Style};

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Init(args) => {
            initialize_shell(&args.shell);
        }
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
        Commands::Check(args) => {
            if let Some(regret) = check_command(&args.command) {
                let mut tty = std::fs::OpenOptions::new()
                    .write(true)
                    .open("/dev/tty")
                    .unwrap();

                writeln!(
                    tty,
                    "{}",
                    "This command is in your regret list.".bold().red()
                )
                .unwrap();
                writeln!(tty, "{} {}", "Reason:".red(), regret.reason.yellow()).unwrap();
                tty.flush().unwrap();

                std::process::exit(1);
            }
        }
    }
}
