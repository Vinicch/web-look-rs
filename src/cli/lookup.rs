use clap::ArgMatches;
use trust_dns_resolver::config::{ResolverConfig, ResolverOpts};
use trust_dns_resolver::Resolver;

pub fn lookup(matches: ArgMatches) {
    let resolver = Resolver::new(ResolverConfig::default(), ResolverOpts::default()).unwrap();
    let address = matches.value_of("address").unwrap();

    match matches.subcommand_name() {
        Some("ip") => {
            let response = resolver.lookup_ip(address).unwrap();

            response
                .iter()
                .for_each(|ip| println!("{}", ip.to_string()));
        }
        Some("mx") => {
            let response = resolver.mx_lookup(address).unwrap();

            response
                .iter()
                .for_each(|record| println!("{}", record.to_string()));
        }
        Some("ns") => {
            let response = resolver.ns_lookup(address).unwrap();

            response
                .iter()
                .for_each(|name| println!("{}", name.to_string()));
        }
        _ => println!("Please specify what you want to lookup (ip, mx, ns).\n\nFor more information try --help")
    }
}
