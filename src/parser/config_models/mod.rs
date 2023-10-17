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
pub struct TlsSettings {
    pub allowInsecure: bool,
    pub certificates: u8,
    pub serverName: String,
    // u8 is a dummy type here
    pub alpn: u8,
    pub enableSessionResumption: bool,
    pub disableSystemRoot: bool,
    pub minVersion: String,
    pub maxVersion: String,
    pub cipherSuites: String,
    pub preferServerCipherSuites: bool,
    pub fingerprint: String,
    pub rejectUnknownSni: bool,
    pub pinnedPeerCertificateChainSha256: u8,
    pub pinnedPeerCertificatePublicKeySha256: u8,
}

#[allow(non_snake_case)]
pub struct StreamSettings {
    pub network: String,
    pub security: String,
    pub tlsSettings: TlsSettings,
}

#[derive(Serialize, Deserialize)]
pub struct Outbound {
    pub settings: OutboundSettings,
    pub protocol: String,
    pub tag: String,
}
