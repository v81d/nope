mod checker;
mod cli;
mod config;
mod init;

use checker::check_command;
use clap::Parser;
use cli::{Cli, Commands, ConfigCommands};
use colored::Colorize;
use config::*;
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

    let all_regrets = list_regrets().unwrap();

    match cli.command {
        Commands::Config(config_args) => match config_args.command {
            ConfigCommands::Threshold(threshold_args) => {
                set_warning_threshold(threshold_args.value).unwrap();
                println!("Warning threshold set to {}.", threshold_args.value);
            }
        },
        Commands::Init(args) => {
            initialize_shell(&args.shell);
        }
        Commands::List => {
            let regrets = all_regrets
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
            let id: usize = all_regrets.len();

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
            print!("Are you sure you want to remove this regret? [Y/n] ");
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
            if let (Some(regret), Some(similarity_score)) = check_command(&args.command) {
                eprintln!(
                    "{}",
                    "A similar command was found in your regrets list."
                        .bold()
                        .red()
                );
                eprintln!("{} {}", "Matched Command:".red(), regret.command.yellow());
                eprintln!("{} {}", "Reason:".red(), regret.reason.get().yellow());
                eprintln!(
                    "{} {}",
                    "Timestamp:".red(),
                    regret.timestamp.to_string().yellow()
                );
                eprintln!(
                    "{} {}",
                    "Similarity Score:".red(),
                    format!("{:.2}", similarity_score).yellow()
                );
                std::process::exit(1);
            }
        }
        Commands::Clear => {
            // Prompt
            println!(
                "{}",
                "WARNING: This is a highly destructive action!".bold().red()
            );
            print!(
                "Are you sure you want to clear {} {}? [y/N] ",
                all_regrets.len(),
                if all_regrets.len() == 1 {
                    "regret"
                } else {
                    "regrets"
                }
            );
            io::stdout().flush().unwrap();

            let mut input = String::new();
            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read user input.");

            if input.trim().eq_ignore_ascii_case("y") {
                clear_regrets().unwrap();
                println!("The regrets list has been cleared.")
            }
        }
    }
}
