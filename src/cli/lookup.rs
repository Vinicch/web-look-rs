use trust_dns_resolver::config::{ResolverConfig, ResolverOpts};
use trust_dns_resolver::Resolver;

use crate::builder::{Cli, Commands};

pub fn lookup(cli: Cli) {
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
