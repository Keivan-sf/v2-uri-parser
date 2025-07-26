use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct VlessUser {
    pub id: String,
    pub encryption: String,
    pub flow: Option<String>,
    pub level: Option<u8>,
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
    pub r#type: Option<String>,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct QuicSettings {
    pub header: Option<NonHeaderObject>,
    pub security: Option<String>,
    pub key: Option<String>,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct GRPCSettings {
    pub authority: Option<String>,
    pub multiMode: Option<bool>,
    pub serviceName: Option<String>,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct KCPSettings {
    pub mtu: Option<u32>,
    pub tti: Option<u32>,
    pub uplinkCapacity: Option<u32>,
    pub downlinkCapacity: Option<u32>,
    pub congestion: Option<bool>,
    pub readBufferSize: Option<u32>,
    pub writeBufferSize: Option<u32>,
    pub seed: Option<String>,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct XHTTPSettings {
    pub host: Option<String>,
    pub path: Option<String>,
    pub mode: Option<String>,
    pub extra: Option<serde_json::Value>,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct RealitySettings {
    pub fingerprint: Option<String>,
    pub serverName: Option<String>,
    pub publicKey: Option<String>,
    pub shortId: Option<String>,
    pub spiderX: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct TCPHeader {
    pub r#type: Option<String>,
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
    pub Host: Option<String>,
    pub acceptProxyProtocol: Option<bool>,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct TlsSettings {
    pub alpn: Option<Vec<String>>,
    pub allowInsecure: bool,
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
    pub network: Option<String>,
    pub security: Option<String>,
    pub tlsSettings: Option<TlsSettings>,
    pub wsSettings: Option<WsSettings>,
    pub tcpSettings: Option<TCPSettings>,
    pub realitySettings: Option<RealitySettings>,
    pub grpcSettings: Option<GRPCSettings>,
    pub quicSettings: Option<QuicSettings>,
    pub kcpSettings: Option<KCPSettings>,
    pub xhttpSettings: Option<XHTTPSettings>,
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
    pub settings: Option<InboundSettings>,
    pub sniffing: Option<SniffingSettings>,
    pub tag: String,
}

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub outbounds: Vec<Outbound>,
    pub inbounds: Vec<Inbound>,
}
