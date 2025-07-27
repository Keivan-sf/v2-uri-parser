pub mod data;
mod models;
use crate::config_models::*;

pub fn create_outbound_settings(data: &RawData) -> OutboundSettings {
    return OutboundSettings::ShadowSocks(ShadowSocksOutboundSettings {
        servers: vec![ShadowSocksServerObject {
            address: data.address.clone(),
            port: data.port,
            password: data.uuid.clone(),
            level: Some(0),
            method: data.server_method.clone(),
        }],
    });
}
