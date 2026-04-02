mod checker;
mod cli;
mod config;
mod init;

use checker::check_command;
use clap::Parser;
use cli::{Cli, Commands};
use colored::Colorize;
use config::{Reason, Regret, Timestamp, add_regret, get_regret, list_regrets, remove_regret};
use init::initialize_shell;
use std::io::{self, Write};
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
            let id: usize = list_regrets().unwrap().len();

            // Regret details
            println!("{}", format!("Regret {}:", id).bold().cyan());
            println!("{} {}", "Command:".cyan(), regret.command.yellow());
            println!("{} {}", "Reason:".cyan(), regret.reason.get().yellow());
            println!(
                "{} {}",
                "Timestamp:".cyan(),
                regret.timestamp.to_string().yellow()
            );

            // Prompt
            print!("Would you like to add this regret? [Y/n] ");
            io::stdout().flush().unwrap(); // force-write buffered output

            let mut input = String::new();
            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read user input.");

            if input.trim().eq_ignore_ascii_case("y") || input.trim().is_empty() {
                add_regret(regret).unwrap();
                println!("The regret has been added.")
            }
        }
        Commands::Remove(args) => {
            let regret: Regret = get_regret(args.id).unwrap();

            // Regret details
            println!("{}", format!("Regret {}:", args.id).bold().cyan());
            println!("{} {}", "Command:".cyan(), regret.command.yellow());
            println!("{} {}", "Reason:".cyan(), regret.reason.get().yellow());
            println!(
                "{} {}",
                "Timestamp:".cyan(),
                regret.timestamp.to_string().yellow()
            );

            // Prompt
            print!("Are you sure you want to remove this entry? [Y/n] ");
            io::stdout().flush().unwrap();

            let mut input = String::new();
            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read user input.");

            if input.trim().eq_ignore_ascii_case("y") || input.trim().is_empty() {
                remove_regret(args.id).unwrap();
                println!("The regret has been removed.")
            }
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
