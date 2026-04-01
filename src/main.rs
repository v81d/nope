mod cli;
mod config;

use clap::Parser;
use cli::{Cli, Commands};

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::List => println!("List something."),
        Commands::Add(args) => println!("{} - {}", args.command, args.description),
        _ => (),
    }
}
