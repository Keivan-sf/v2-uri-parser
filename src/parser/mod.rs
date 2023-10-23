use std::process::exit;
pub mod config_models;
mod uri_identifier;
mod vless;
pub fn parse(uri: &str) {
    let protocol = uri_identifier::get_uri_protocol(uri);
    let mut serialized: String = String::from("");
    match protocol {
        Some(uri_identifier::Protocols::Vless) => {
            println!("The protocol is Vless");
            let vless_data = vless::get_vless_data(uri);
            let outbound_object = vless::create_outbound_object(vless_data);
            serialized = serde_json::to_string(&outbound_object).unwrap();
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
    println!("The parsed config is :\n{}", serialized);
}
