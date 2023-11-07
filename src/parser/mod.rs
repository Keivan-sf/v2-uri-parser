use std::process::exit;

use self::config_models::{Config, Outbound};
pub mod config_models;
mod uri_identifier;
mod vless;

pub fn create_json_config(uri: &str, socks_port: Option<u16>) -> String {
    let config = create_config(uri, socks_port);
    let serialized = serde_json::to_string(&config).unwrap();
    return serialized;
}

pub fn create_config(uri: &str, socks_port: Option<u16>) -> Config {
    let outbound_object = create_outbound_object(uri);
    let config = Config {
        outbounds: vec![outbound_object],
        inbounds: None,
    };
    return config;
}

pub fn create_outbound_object(uri: &str) -> Outbound {
    let protocol = uri_identifier::get_uri_protocol(uri);
    match protocol {
        Some(uri_identifier::Protocols::Vless) => {
            println!("The protocol is Vless");
            let vless_data = vless::get_vless_data(uri);
            let outbound_object = vless::create_outbound_object(vless_data);
            return outbound_object;
        }
        Some(_) => {
            println!("The protocol was recognized but is not supported yet");
            exit(0);
        }
        None => {
            println!("The protcol is not supported");
            exit(0);
        }
    }
}
