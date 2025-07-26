#[allow(non_snake_case)]
pub struct VlessQuery {
    pub security: Option<String>,
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
