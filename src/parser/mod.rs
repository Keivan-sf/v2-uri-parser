use std::process::exit;
pub mod config_models;
mod uri_identifier;
mod vless;
pub fn parse(uri: &str) {
    let protocol = uri_identifier::get_uri_protocol(uri);
    match protocol {
        Some(uri_identifier::Protocols::Vmess) => {
            println!("The protocol was vmess");
        }
        Some(uri_identifier::Protocols::Vless) => {
            println!("The protocol is Vless");
            let vless_data = vless::get_vless_data(uri);
            let outbound_object = vless::create_outbound_object(vless_data);
            let serialized = serde_json::to_string(&outbound_object).unwrap();
            println!("The parsed config is :\n{}", serialized);
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
