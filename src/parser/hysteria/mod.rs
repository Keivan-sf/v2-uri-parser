pub mod data;
mod models;
use crate::config_models::*;

pub fn create_outbound_settings(data: &RawData) -> OutboundSettings {
    return OutboundSettings::Hysteria(HysteriaOutboundSettings {
        address: data.address.clone(),
        port: data.port,
        version: Some(2),
    });
}
