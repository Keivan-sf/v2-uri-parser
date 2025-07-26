pub struct VlessQuery {
    pub security: String,
    pub sni: String,
    pub fp: String,
    pub pbk: String,
    pub sid: String,
    pub r#type: String,
    pub flow: String,
    pub path: String,
    pub encryption: String,
    pub header_type: String,
    pub host: String,
    pub seed: String,
    pub quic_security: String,
    pub r#key: String,
    pub mode: String,
    pub service_name: String,
    pub slpn: String,
    pub spx: String,
    pub alpn: String,
}

pub struct VlessAddress {
    pub uuid: String,
    pub address: String,
    pub port: u16,
}

pub struct VlessData {
    pub query: VlessQuery,
    pub address_data: VlessAddress,
}
