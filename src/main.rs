use std::process::exit;

mod parser;
fn main() {
    let protocol = parser::get_uri_protocol("vmess://");
    match protocol {
        Some(parser::Protocols::Vless) => {
            println!("The protocol was Vless");
        }
        Some(_) => {
            println!("Some recognizable protocol")
        }
        None => {
            println!("The protocol is not supported");
            exit(0);
        }
    }
}
