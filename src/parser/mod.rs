use std::process::exit;
mod uri_identifier;
mod vless;
pub mod config_models;
pub fn parse(uri: &str) {
    let protocol = uri_identifier::get_uri_protocol(uri);
    match protocol {
        Some(uri_identifier::Protocols::Vmess) => {
            println!("The protocol was vmess");
        }
        Some(_) => {
            println!("The protocol was recognized");
        }
        None => {
            println!("The protcol is not supported");
            exit(0);
        }
    }
}
