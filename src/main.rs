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
use std::io::{self, Read, Write};
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

fn read_stdin_command() -> io::Result<String> {
    let mut buffer = String::new();
    let mut stdin = io::stdin();
    stdin.read_to_string(&mut buffer)?;
    Ok(buffer.trim_end_matches("\n").to_string())
}

fn main() {
    let cli = Cli::parse();

    let all_regrets = list_regrets().unwrap();

    match cli.command {
        Commands::Config(config_args) => match config_args.command {
            ConfigCommands::Enable => {
                set_enabled(true).unwrap();
                println!("Checks are now enabled. Warnings will appear on scans.");
            }
            ConfigCommands::Disable => {
                set_enabled(false).unwrap();
                println!("Checks are now disabled. No warning will appear on scans.");
            }
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

            println!(
                "{}",
                "WARNING: All commands in the regret list are stored in plaintext form. Do not add any sensitive information. Confirm that your command does not contain secrets."
                    .bold()
                    .red()
            );

            // ask user to verify that there are no secrets
            // some users might not read the notice, so we should assume the worst case (that is,
            // the command DOES contain secrets)
            print!("Does your command contain secrets? [Y/n] ");
            io::stdout().flush().unwrap();

            let mut confirmation = String::new();
            io::stdin()
                .read_line(&mut confirmation)
                .expect("Failed to read user input.");

            if confirmation.trim().eq_ignore_ascii_case("y") || confirmation.trim().is_empty() {
                println!(
                    "{}",
                    "Cannot add this command because it contains secrets.".red()
                );
                return;
            }

            // show regret details
            println!("\n{}", format!("Regret {}:", id).bold().cyan());
            println!("{} {}", "Command:".cyan(), regret.command.yellow());
            println!("{} {}", "Reason:".cyan(), regret.reason.get().yellow());
            println!(
                "{} {}",
                "Timestamp:".cyan(),
                regret.timestamp.to_string().yellow()
            );

            // confirm before adding the regret
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

            println!("{}", format!("Regret {}:", args.id).bold().cyan());
            println!("{} {}", "Command:".cyan(), regret.command.yellow());
            println!("{} {}", "Reason:".cyan(), regret.reason.get().yellow());
            println!(
                "{} {}",
                "Timestamp:".cyan(),
                regret.timestamp.to_string().yellow()
            );

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
            let command: String = match args.command {
                Some(c) => c, // if the user ran the command manually with the command to check as
                // the input, we should still accept it assuming the user understands the risks
                None => {
                    read_stdin_command() // (for shell hooks) we should read from stdin so the full
                        // command (which might contain secrets) doesn't get leaked to the process list
                        .unwrap_or_else(|_| String::new())
                }
            };

            if command.is_empty() {
                return;
            }

            if let (Some(regret), Some(similarity_score)) = check_command(&command) {
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
            println!(
                "{}",
                "WARNING: This is a highly destructive action! Think before you type."
                    .bold()
                    .red()
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
