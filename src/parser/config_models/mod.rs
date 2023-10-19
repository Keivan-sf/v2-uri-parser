use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct VlessUser {
    pub id: String,
    pub encryption: String,
    pub flow: String,
    pub level: u8,
}

#[derive(Serialize, Deserialize)]
pub struct VlessServerObject {
    pub address: String,
    pub port: u16,
    pub users: Vec<VlessUser>,
}

#[derive(Serialize, Deserialize)]
pub struct VlessOutboundSettings {
    pub vnext: Vec<VlessServerObject>,
}

#[derive(Serialize, Deserialize)]
pub enum OutboundSettings {
    Vless(VlessOutboundSettings),
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct TlsSettings {
    pub allowInsecure: Option<bool>,
    pub serverName: Option<String>,
    pub enableSessionResumption: Option<bool>,
    pub disableSystemRoot: Option<bool>,
    pub minVersion: Option<String>,
    pub maxVersion: Option<String>,
    pub cipherSuites: Option<String>,
    pub preferServerCipherSuites: Option<bool>,
    pub fingerprint: Option<String>,
    pub rejectUnknownSni: Option<bool>,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct StreamSettings {
    pub network: String,
    pub security: String,
    pub tlsSettings: Option<TlsSettings>,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Outbound {
    pub settings: OutboundSettings,
    pub streamSettings: StreamSettings,
    pub protocol: String,
    pub tag: String,
}
