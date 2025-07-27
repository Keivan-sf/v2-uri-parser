pub mod data;
mod models;
use crate::config_models::*;

pub fn create_outbound_settings(data: &RawData) -> OutboundSettings {
    return OutboundSettings::Trojan(TrojanOutboundSettings {
        servers: vec![TrojanServerObject {
            address: data.address.clone(),
            port: data.port,
            password: data.uuid.clone(),
            level: Some(0),
        }],
    });
}
