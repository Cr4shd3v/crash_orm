mod migrate;
mod init;

use crate::command::migrate::Migrate;
use clap::Subcommand;
use crate::command::init::Init;

#[derive(Subcommand, Debug)]
pub enum Command {
    Init(Init),
    Migrate(Migrate),
}

impl Command {
    pub fn run(self) {
        match self {
            Command::Init(init) => init.run(),
            Command::Migrate(migrate) => migrate.run(),
        }
    }
}