use clap::{Parser, Subcommand};
use trust_dns_resolver::config::{ResolverConfig, ResolverOpts};
use trust_dns_resolver::Resolver;

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
    #[command(about = "Looks up the DNS Mail eXchanger records for a particular host")]
    Mx,
    #[command(about = "Looks up the Name Servers for a particular host")]
    Ns,
}

fn main() {
    let cli = Cli::parse();
    let resolver = Resolver::new(ResolverConfig::default(), ResolverOpts::default()).unwrap();

    match &cli.command {
        Commands::Ip => {
            let response = resolver.lookup_ip(cli.address).unwrap();

            for ip in response {
                println!("{ip}")
            }
        }
        Commands::Mx => {
            let response = resolver.mx_lookup(cli.address).unwrap();

            for record in response {
                println!("{record}")
            }
        }
        Commands::Ns => {
            let response = resolver.ns_lookup(cli.address).unwrap();

            for name in response {
                println!("{name}")
            }
        }
    }
}
