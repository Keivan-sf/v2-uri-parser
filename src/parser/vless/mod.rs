pub mod data;
mod models;
use crate::config_models::*;

pub fn create_outbound_settings(data: &RawData) -> OutboundSettings {
    return OutboundSettings::Vless(VlessOutboundSettings {
        vnext: vec![VnextServerObject {
            port: data.port,
            address: data.address.clone(),
            users: Some(vec![VnextUser {
                id: data.uuid.clone(),
                flow: data.flow.clone(),
                encryption: Some(data.encryption.clone().unwrap_or(String::from("none"))),
                level: Some(0),
                security: None,
            }]),
        }],
    });
}
