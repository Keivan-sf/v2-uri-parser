use crate::config_models::{
    self, GRPCSettings, KCPSettings, NonHeaderObject, Outbound, OutboundSettings, QuicSettings,
    RawData, RealitySettings, StreamSettings, TCPHeader, TCPSettings, TlsSettings, WsSettings,
    XHTTPSettings,
};
use crate::utils::{inbound_generator, parse_raw_json};

mod uri_identifier;
mod vless;
mod vmess;

pub fn get_name(uri: &str) -> String {
    let (_, data, _) = get_uri_data(uri);
    return data.remarks;
}

pub fn create_json_config(uri: &str, socks_port: Option<u16>, http_port: Option<u16>) -> String {
    let config = create_config(uri, socks_port, http_port);
    let serialized = serde_json::to_string(&config).unwrap();
    return serialized;
}

pub fn create_config(
    uri: &str,
    socks_port: Option<u16>,
    http_port: Option<u16>,
) -> config_models::Config {
    let outbound_object = create_outbound_object(uri);
    let inbound_config =
        inbound_generator::generate_inbound_config(inbound_generator::InboundGenerationOptions {
            socks_port,
            http_port,
        });
    let config = config_models::Config {
        outbounds: vec![outbound_object],
        inbounds: inbound_config,
    };
    return config;
}

pub fn create_outbound_object(uri: &str) -> config_models::Outbound {
    let (name, data, outbound_settings) = get_uri_data(uri);

    let network_type = data.r#type.clone().unwrap_or(String::from(""));
    let allow_insecure = data.allowInsecure == Some(String::from("true"))
        || data.allowInsecure == Some(String::from("1"));

    let outbound = Outbound {
        protocol: name,
        tag: String::from("proxy"),
        streamSettings: StreamSettings {
            network: data.r#type.clone(),
            security: data.security.clone(),
            tlsSettings: if data.security == Some(String::from("tls")) {
                Some(TlsSettings {
                    alpn: data.alpn.map(|alpn| vec![alpn]),
                    rejectUnknownSni: None,
                    enableSessionResumption: None,
                    minVersion: None,
                    maxVersion: None,
                    cipherSuites: None,
                    disableSystemRoot: None,
                    preferServerCipherSuites: None,
                    fingerprint: data.fp.clone(),
                    serverName: data.sni.clone(),
                    allowInsecure: allow_insecure,
                })
            } else {
                None
            },
            wsSettings: if network_type == String::from("ws") {
                Some(WsSettings {
                    Host: data.host.clone(),
                    path: data.path.clone(),
                    acceptProxyProtocol: None,
                })
            } else {
                None
            },
            tcpSettings: if network_type == String::from("tcp") {
                Some(TCPSettings {
                    header: Some(TCPHeader {
                        r#type: Some(data.header_type.unwrap_or(String::from("none"))),
                    }),
                    acceptProxyProtocol: None,
                })
            } else {
                None
            },
            realitySettings: if network_type == String::from("reality") {
                Some(RealitySettings {
                    publicKey: data.pbk,
                    serverName: data.sni.clone(),
                    shortId: data.sid,
                    spiderX: Some(String::from("")),
                    fingerprint: data.fp.clone(),
                })
            } else {
                None
            },
            grpcSettings: if network_type == String::from("grpc") {
                Some(GRPCSettings {
                    authority: data.authority,
                    multiMode: Some(false),
                    serviceName: data.service_name,
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
                    seed: data.seed,
                })
            } else {
                None
            },
            xhttpSettings: if network_type == String::from("xhttp") {
                Some(XHTTPSettings {
                    host: data.host.clone(),
                    path: data.path.clone(),
                    mode: data.mode,
                    extra: data.extra.and_then(|e| parse_raw_json(e.as_str())),
                })
            } else {
                None
            },
        },
        settings: outbound_settings,
    };

    return outbound;
}

fn get_uri_data(uri: &str) -> (String, RawData, OutboundSettings) {
    let protocol = uri_identifier::get_uri_protocol(uri);
    return match protocol {
        Some(uri_identifier::Protocols::Vless) => {
            let d = vless::data::get_data(uri);
            let s = vless::create_outbound_settings(&d);
            (String::from("vless"), d, s)
        }
        Some(uri_identifier::Protocols::Vmess) => {
            let d = vmess::data::get_data(uri);
            let s = vmess::create_outbound_settings(&d);
            (String::from("vmess"), d, s)
        }
        Some(_) => {
            panic!("The protocol was recognized but is not supported yet");
        }
        None => {
            panic!("The protocol is not supported");
        }
    };
}
