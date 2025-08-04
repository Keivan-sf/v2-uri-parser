use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct VnextUser {
    pub id: Option<String>,
    pub encryption: Option<String>,
    pub flow: Option<String>,
    pub level: Option<u8>,
    pub security: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct SocksUser {
    pub user: Option<String>,
    pub pass: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct VnextServerObject {
    pub address: Option<String>,
    pub port: Option<u16>,
    pub users: Option<Vec<VnextUser>>,
}

#[derive(Serialize, Deserialize)]
pub struct TrojanServerObject {
    pub address: Option<String>,
    pub port: Option<u16>,
    pub password: Option<String>,
    pub level: Option<u8>,
}

#[derive(Serialize, Deserialize)]
pub struct ShadowSocksServerObject {
    pub address: Option<String>,
    pub port: Option<u16>,
    pub password: Option<String>,
    pub level: Option<u8>,
    pub method: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct SocksServerObject {
    pub address: Option<String>,
    pub port: Option<u16>,
    pub level: Option<u8>,
    pub users: Option<Vec<SocksUser>>,
}

#[derive(Serialize, Deserialize)]
pub struct VlessOutboundSettings {
    pub vnext: Vec<VnextServerObject>,
}

#[derive(Serialize, Deserialize)]
pub struct VmessOutboundSettings {
    pub vnext: Vec<VnextServerObject>,
}

#[derive(Serialize, Deserialize)]
pub struct TrojanOutboundSettings {
    pub servers: Vec<TrojanServerObject>,
}

#[derive(Serialize, Deserialize)]
pub struct ShadowSocksOutboundSettings {
    pub servers: Vec<ShadowSocksServerObject>,
}

#[derive(Serialize, Deserialize)]
pub struct SocksOutboundSettings {
    pub servers: Vec<SocksServerObject>,
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum OutboundSettings {
    Vless(VlessOutboundSettings),
    Vmess(VmessOutboundSettings),
    Trojan(TrojanOutboundSettings),
    ShadowSocks(ShadowSocksOutboundSettings),
    Socks(SocksOutboundSettings),
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

#[derive(Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct RawData {
    pub remarks: String,
    pub security: Option<String>,
    pub vnext_security: Option<String>,
    pub sni: Option<String>,
    pub fp: Option<String>,
    pub pbk: Option<String>,
    pub sid: Option<String>,
    pub r#type: Option<String>,
    pub flow: Option<String>,
    pub path: Option<String>,
    pub encryption: Option<String>,
    pub header_type: Option<String>,
    pub host: Option<String>,
    pub seed: Option<String>,
    pub quic_security: Option<String>,
    pub r#key: Option<String>,
    pub mode: Option<String>,
    pub service_name: Option<String>,
    pub authority: Option<String>,
    pub slpn: Option<String>,
    pub spx: Option<String>,
    pub alpn: Option<String>,
    pub extra: Option<String>,
    pub allowInsecure: Option<String>,
    pub uuid: Option<String>,
    pub address: Option<String>,
    pub port: Option<u16>,
    pub server_method: Option<String>,
    pub username: Option<String>,
}

#[derive(Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct ConfigMetaData {
    pub name: String,
    pub protocol: String,
    pub host: Option<String>,
    pub address: Option<String>,
    pub port: Option<u16>,
}
