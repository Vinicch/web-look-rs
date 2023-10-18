mod cli;

use clap::Parser;
use cli::{builder, lookup};

fn main() {
    let cli = builder::Cli::parse();

    lookup::lookup(cli);
}
