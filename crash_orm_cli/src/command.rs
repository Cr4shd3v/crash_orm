mod migrate;

use crate::command::migrate::Migrate;
use clap::Subcommand;

#[derive(Subcommand, Debug)]
pub enum Command {
    Migrate(Migrate),
}

impl Command {
    pub fn run(self) {
        match self {
            Command::Migrate(migrate) => migrate.run(),
        }
    }
}