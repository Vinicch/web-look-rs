use clap::ArgMatches;
use trust_dns_resolver::config::{ResolverConfig, ResolverOpts};
use trust_dns_resolver::Resolver;

pub fn lookup(matches: ArgMatches) {
    let resolver = Resolver::new(ResolverConfig::default(), ResolverOpts::default()).unwrap();
    let address = matches.value_of("address").unwrap();

    match matches.subcommand_name() {
        Some("ip") => {
            let response = resolver.lookup_ip(address).unwrap();

            for ip in response {
                println!("{ip}")
            }
        }
        Some("mx") => {
            let response = resolver.mx_lookup(address).unwrap();

            for record in response {
                println!("{record}")
            }
        }
        Some("ns") => {
            let response = resolver.ns_lookup(address).unwrap();

            for name in response {
                println!("{name}")
            }
        }
        _ => println!("Please specify what you want to lookup (ip, mx, ns).\n\nFor more information try --help")
    }
}
