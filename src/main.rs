mod parser;
use clap::Parser;
pub mod config_models;
pub mod utils;

#[derive(Parser)]
#[command(author ,version = "0.1.1", about = "V2ray URI parser", long_about = None)]
struct Cli {
    uri: String,
    #[arg(short, long, value_name = "socksport")]
    socksport: Option<u16>,
    #[arg(long, value_name = "httpport")]
    httpport: Option<u16>,
}

fn main() {
    let cli = Cli::parse();
    let json_config = parser::create_json_config(&cli.uri, cli.socksport, cli.httpport);
    println!("{}", json_config);
}
