use clap::{App, Arg, SubCommand};

pub fn build_app() -> App<'static, 'static> {
    let args = build_args();
    let subcommands = build_subcommands();

    App::new("Web Look")
        .version("0.1.0")
        .author("Vinicius Leme <vibalta@gmail.com>")
        .about("Let's you query IPs, MX records and Name Servers!")
        .args(&args)
        .subcommands(subcommands)
}

fn build_args() -> Vec<Arg<'static, 'static>> {
    vec![Arg::with_name("address")
        .short("a")
        .long("address")
        .help("Sets the Host Address that's going to be looked up")
        .takes_value(true)
        .required(true)]
}

fn build_subcommands() -> Vec<App<'static, 'static>> {
    vec![
        SubCommand::with_name("ip").about("Looks up the IP addresses for a particular host"),
        SubCommand::with_name("mx").about("Looks up the DNS MX records for a particular host"),
        SubCommand::with_name("ns")
            .about("Looks up the Name Servers for a particular host"),
    ]
}
