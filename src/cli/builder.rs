use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(version, about = "Lookup your address information!")]
pub struct Cli {
    #[arg(help = "Address that is going to be looked up")]
    pub address: String,

    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    #[command(about = "Looks up the IP addresses for a particular host")]
    Ip,
    #[command(about = "Looks up the DNS MX records for a particular host")]
    Mx,
    #[command(about = "Looks up the Name Servers for a particular host")]
    Ns,
}
