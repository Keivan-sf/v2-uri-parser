use clap::{value_parser, Arg, Command};
pub mod config_models;
mod parser;
pub mod utils;

fn main() {
    let matches = Command::new("v2parser")
        .version("0.2.0")
        .about("Parses V2ray URI and generates JSON config for xray")
        .arg(
            Arg::new("uri")
                .help("V2ray URI to parse")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::new("socksport")
                .long("socksport")
                .help("Optional SOCKS5 proxy port for inbound")
                .value_name("PORT")
                .value_parser(value_parser!(u16)),
        )
        .arg(
            Arg::new("httpport")
                .long("httpport")
                .help("Optional HTTP proxy port for inbound")
                .value_name("PORT")
                .value_parser(value_parser!(u16)),
        )
        .arg(
            Arg::new("get_name")
                .long("get-name")
                .help("Only print the config name")
                .action(clap::ArgAction::SetTrue),
        )
        .get_matches();

    let uri = matches.get_one::<String>("uri").unwrap();
    let socksport = matches.get_one::<u16>("socksport").copied();
    let httpport = matches.get_one::<u16>("httpport").copied();
    let get_name = matches.get_flag("get_name");

    if get_name {
        print!("{}", parser::get_name(uri));
        return;
    }

    let json_config = parser::create_json_config(uri, socksport, httpport);
    println!("{}", json_config);
}
