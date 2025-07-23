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
#[serde(untagged)]
pub enum OutboundSettings {
    Vless(VlessOutboundSettings),
}

#[derive(Serialize, Deserialize)]
pub struct NonHeaderObject {
    pub r#type: String,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct QuicSettings {
    pub header: Option<NonHeaderObject>,
    pub security: String,
    pub key: String,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct GRPCSettings {
    pub multiMode: bool,
    pub serviceName: String,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct RealitySettings {
    pub fingerprint: String,
    pub serverName: String,
    pub publicKey: String,
    pub shortId: String,
    pub spiderX: String,
}

#[derive(Serialize, Deserialize)]
pub struct TCPHeader {
    pub r#type: String,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct TCPSettings {
    pub header: Option<TCPHeader>,
    pub acceptProxyProtocol: Option<bool>,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct HeaderSetting {
    pub Host: Option<String>,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct WsSettings {
    pub path: Option<String>,
    pub headers: Option<HeaderSetting>,
    pub acceptProxyProtocol: Option<bool>,
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
    pub wsSettings: Option<WsSettings>,
    pub tcpSettings: Option<TCPSettings>,
    pub realitySettings: Option<RealitySettings>,
    pub grpcSettings: Option<GRPCSettings>,
    pub quicSettings: Option<QuicSettings>,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Outbound {
    pub settings: OutboundSettings,
    pub streamSettings: StreamSettings,
    pub protocol: String,
    pub tag: String,
}

#[derive(Serialize, Deserialize)]
pub struct InboundSettings {
    pub udp: bool,
}

#[derive(Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct SniffingSettings {
    pub enabled: Option<bool>,
    pub destOverride: Option<Vec<String>>,
    pub domainsExcluded: Option<Vec<String>>,
    pub metadataOnly: Option<bool>,
    pub routeOnly: Option<bool>,
}

#[derive(Serialize, Deserialize)]
pub struct Inbound {
    pub listen: String,
    pub port: u16,
    pub protocol: String,
    pub settings: InboundSettings,
    pub sniffing: Option<SniffingSettings>,
    pub tag: String,
}

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub outbounds: Vec<Outbound>,
    pub inbounds: Vec<Inbound>,
}
