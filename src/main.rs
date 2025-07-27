mod parser;
use clap::Parser;
pub mod config_models;
pub mod utils;

#[derive(Parser)]
#[command(author ,version = "0.1.1", about = "V2ray URI parser", long_about = None)]
struct Cli {
    uri: String,
    #[arg(long, value_name = "socksport")]
    socksport: Option<u16>,
    #[arg(long, value_name = "httpport")]
    httpport: Option<u16>,
    #[arg(long, action = clap::ArgAction::SetTrue, value_name = "get-name")]
    get_name: Option<bool>,
}

fn main() {
    let cli = Cli::parse();
    if cli.get_name == Some(true) {
        print!("{}", parser::get_name(&cli.uri));
        return;
    }
    let json_config = parser::create_json_config(&cli.uri, cli.socksport, cli.httpport);
    println!("{}", json_config);
}
