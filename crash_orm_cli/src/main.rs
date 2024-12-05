mod command;

use crate::command::Command;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

fn main() {
    dotenvy::dotenv().ok();

    let cli = Cli::parse();

    cli.command.run();
}
