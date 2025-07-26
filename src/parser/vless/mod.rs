pub mod data;
mod models;
use crate::{config_models::*, utils::parse_raw_json};

pub fn create_outbound_object(data: models::VlessData) -> Outbound {
    let network_type = data.query.r#type.clone().unwrap_or(String::from(""));
    return Outbound {
        protocol: String::from("vless"),
        tag: String::from("proxy"),
        streamSettings: StreamSettings {
            network: data.query.r#type.clone(),
            security: data.query.security.clone(),
            tlsSettings: if data.query.security == Some(String::from("tls")) {
                Some(TlsSettings {
                    alpn: data.query.alpn.map(|alpn| vec![alpn]),
                    rejectUnknownSni: None,
                    enableSessionResumption: None,
                    minVersion: None,
                    maxVersion: None,
                    cipherSuites: None,
                    disableSystemRoot: None,
                    preferServerCipherSuites: None,
                    fingerprint: data.query.fp.clone(),
                    serverName: data.query.sni.clone(),
                    allowInsecure: Some(false),
                })
            } else {
                None
            },
            wsSettings: if network_type == String::from("ws") {
                Some(WsSettings {
                    Host: data.query.host.clone(),
                    path: data.query.path.clone(),
                    acceptProxyProtocol: None,
                })
            } else {
                None
            },
            tcpSettings: if network_type == String::from("tcp") {
                Some(TCPSettings {
                    header: Some(TCPHeader {
                        r#type: Some(data.query.header_type.unwrap_or(String::from("none"))),
                    }),
                    acceptProxyProtocol: None,
                })
            } else {
                None
            },
            realitySettings: if network_type == String::from("reality") {
                Some(RealitySettings {
                    publicKey: data.query.pbk,
                    serverName: data.query.sni.clone(),
                    shortId: data.query.sid,
                    spiderX: Some(String::from("")),
                    fingerprint: data.query.fp.clone(),
                })
            } else {
                None
            },
            grpcSettings: if network_type == String::from("grpc") {
                Some(GRPCSettings {
                    authority: data.query.authority,
                    multiMode: Some(false),
                    serviceName: data.query.service_name,
                })
            } else {
                None
            },
            quicSettings: if network_type == String::from("quic") {
                Some(QuicSettings {
                    header: Some(NonHeaderObject {
                        r#type: Some(String::from("none")),
                    }),
                    security: Some(String::from("none")),
                    key: Some(String::from("")),
                })
            } else {
                None
            },
            kcpSettings: if network_type == String::from("kcp") {
                Some(KCPSettings {
                    mtu: None,
                    tti: None,
                    congestion: None,
                    uplinkCapacity: None,
                    readBufferSize: None,
                    writeBufferSize: None,
                    downlinkCapacity: None,
                    seed: data.query.seed,
                })
            } else {
                None
            },
            xhttpSettings: if network_type == String::from("xhttp") {
                Some(XHTTPSettings {
                    host: data.query.host.clone(),
                    path: data.query.path.clone(),
                    mode: data.query.mode,
                    extra: data.query.extra.and_then(|e| parse_raw_json(e.as_str())),
                })
            } else {
                None
            },
        },
        settings: OutboundSettings::Vless(VlessOutboundSettings {
            vnext: vec![VlessServerObject {
                port: data.address_data.port,
                address: data.address_data.address,
                users: vec![VlessUser {
                    id: data.address_data.uuid,
                    flow: data.query.flow,
                    encryption: data.query.encryption.unwrap_or(String::from("none")),
                    level: Some(0),
                }],
            }],
        }),
    };
}
