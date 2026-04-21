use clap::{value_parser, Arg, Command};
pub mod config_models;
mod parser;
pub mod utils;

fn main() {
    let matches = Command::new("v2parser")
        .version("0.3.1")
        .about("Parses V2ray URI and generates JSON config for xray")
        .arg(
            Arg::new("uri")
                .help("V2ray URI to parse")
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
            Arg::new("get_metadata")
                .long("get-metadata")
                .help("Only print config meta data")
                .action(clap::ArgAction::SetTrue),
        )
        .get_matches();

    let uri = match matches.get_one::<String>("uri") {
        Some(uri) => Some(uri.to_owned()),
        None => dialoguer::Input::new().interact_text().ok()
    }.unwrap();
    let socksport = matches.get_one::<u16>("socksport").copied();
    let httpport = matches.get_one::<u16>("httpport").copied();
    let get_metadata = matches.get_flag("get_metadata");

    if get_metadata {
        print!("{}", parser::get_metadata(uri.as_str()));
        return;
    }

    let json_config = parser::create_json_config(uri.as_str(), socksport, httpport);
    println!("{}", json_config);
}
