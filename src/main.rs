mod parser;
use clap::{Parser, Subcommand};
use std::path::PathBuf;

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
    // let args: Vec<String> = std::env::args().collect();
    // let uri = args.get(1).unwrap();
    // parser::parse(uri);
}
