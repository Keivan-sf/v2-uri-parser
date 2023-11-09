mod parser;
use clap::Parser;
pub mod config_models;
pub mod utils;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    uri: String,
    #[arg(short, long, value_name = "socksport")]
    socksport: Option<u16>,
}

fn main() {
    let cli = Cli::parse();
    match cli.socksport {
        Some(port) => {
            println!("the port: {}", port)
        }
        None => {
            println!("the port is not here")
        }
    };
    println!("the uri is: {}", cli.uri);
    let json_config = parser::create_json_config(&cli.uri, cli.socksport);
    println!("The json config is: {}", json_config);
}
