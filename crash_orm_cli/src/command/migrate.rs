use clap::Parser;

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

    }
}