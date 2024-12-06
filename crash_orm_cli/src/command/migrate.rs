use clap::Parser;
use crate::config::CrashOrmToml;

#[derive(Parser, Debug)]
pub struct Migrate {
    name: String,

    #[clap(long)]
    empty: bool,

    #[clap(long, short)]
    dry: bool,
}

impl Migrate {
    pub fn run(self) {
        let Some(_) = CrashOrmToml::try_load() else {
            println!("Project is not initialized! Please initialize the project with `crash_orm_cli init`");
            return;
        };
    }
}