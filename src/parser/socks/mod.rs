pub mod data;
mod models;
use crate::config_models::*;

pub fn create_outbound_settings(data: &RawData) -> OutboundSettings {
    return OutboundSettings::Socks(SocksOutboundSettings {
        servers: vec![SocksServerObject {
            users: match (&data.username, &data.uuid) {
                (Some(username), Some(uuid)) => Some(vec![SocksUser {
                    user: Some(username.clone()),
                    pass: Some(uuid.clone()),
                }]),
                _ => None,
            },
            address: data.address.clone(),
            port: data.port,
            level: Some(0),
        }],
    });
}
