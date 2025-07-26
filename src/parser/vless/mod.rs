use querystring;
mod models;
use crate::config_models::*;
use http::Uri;
use std::process::exit;

pub fn create_outbound_object(data: models::VlessData) -> Outbound {
    let network_type = data.query.r#type.clone().unwrap_or(String::from(""));
    return Outbound {
        protocol: String::from("vless"),
        tag: String::from("proxy"),
        streamSettings: StreamSettings {
            network: data.query.r#type.clone(),
            security: data.query.security.clone(),
            tlsSettings: if network_type == String::from("tls") {
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
                    Host: data.query.host,
                    path: data.query.path,
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

pub fn get_vless_data(uri: &str) -> models::VlessData {
    let data = uri.split_once("vless://").unwrap().1;
    let query_and_name = uri.split_once("?").unwrap().1;
    let query = query_and_name
        .split_once("#")
        .unwrap_or((query_and_name, ""))
        .0;
    let parsed_query = parse_vless_query(query);
    let parsed_address = parse_vless_address(data.split_once("?").unwrap().0);
    return models::VlessData {
        query: parsed_query,
        address_data: parsed_address,
    };
}

fn parse_vless_address(raw_data: &str) -> models::VlessAddress {
    let (uuid, raw_address): (String, &str) = match raw_data.split_once("@") {
        None => {
            println!("Wrong vless format, no `@` found in the address");
            exit(0);
        }
        Some(data) => (String::from(data.0), data.1),
    };
    let address_wo_slash = raw_address.strip_suffix("/").unwrap_or(raw_address);

    let parsed = address_wo_slash.parse::<Uri>().unwrap();

    return models::VlessAddress {
        uuid: url_decode(Some(uuid)).unwrap(),
        address: parsed.host().unwrap().to_string(),
        port: parsed.port().unwrap().as_u16(),
    };
}

fn parse_vless_query(raw_query: &str) -> models::VlessQuery {
    let query: Vec<(&str, &str)> = querystring::querify(raw_query);

    let a = models::VlessQuery {
        alpn: url_decode(get_parameter_value(&query, "alpn")),
        path: url_decode(get_parameter_value(&query, "path")),
        authority: url_decode(get_parameter_value(&query, "authority")),
        pbk: url_decode(get_parameter_value(&query, "pbk")),
        security: get_parameter_value(&query, "security"),
        sid: url_decode(get_parameter_value(&query, "sid")),
        flow: get_parameter_value(&query, "flow"),
        sni: get_parameter_value(&query, "sni"),
        fp: url_decode(get_parameter_value(&query, "fp")),
        r#type: get_parameter_value(&query, "type"),
        encryption: get_parameter_value(&query, "encryption"),
        header_type: get_parameter_value(&query, "headerType"),
        host: url_decode(get_parameter_value(&query, "host")),
        seed: url_decode(get_parameter_value(&query, "seed")),
        quic_security: get_parameter_value(&query, "quicSecurity"),
        key: get_parameter_value(&query, "key"),
        mode: url_decode(get_parameter_value(&query, "mode")),
        service_name: url_decode(get_parameter_value(&query, "serviceName")),
        slpn: get_parameter_value(&query, "slpn"),
        spx: url_decode(get_parameter_value(&query, "spx")),
    };
    return a;
}

fn url_decode(value: Option<String>) -> Option<String> {
    return value.and_then(|s| {
        urlencoding::decode(&s)
            .ok()
            .map(|decoded| decoded.into_owned())
    });
}

fn get_parameter_value(query: &Vec<(&str, &str)>, param: &str) -> Option<String> {
    return query
        .iter()
        .find(|q| String::from(q.0) == String::from(param))
        .map(|q| q.1.to_string());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vless_tcp_header_test() {
        let v = "vless://1010501a-ca9a-479c-84d0-1308d97789b5@104.21.25.109:443?security=reality&alpn=http%2F1.1&encryption=none&pbk=testpub&host=mehr14-n.gowow31220.workers.dev&headerType=http&fp=chrome&spx=spideex&type=tcp&flow=xtls-rprx-vision&sni=mehr14-iran-mehr14-iran-mehr14-iran-mehr14-iran-mehr14-iran.gowow31220.workers.dev&sid=testshort#%E2%AD%90%EF%B8%8F%20Telegram%20%3D%20%40z_v2ray";
        let data = create_outbound_object(get_vless_data(v));
        assert_eq!(
            data.streamSettings
                .tcpSettings
                .unwrap()
                .header
                .unwrap()
                .r#type,
            Some(String::from("http"))
        )
    }

    #[test]
    fn vless_test_2() {
        let v = "vless://2dc56709-sdfs-sdfs-2234-128904@nwarne.fast-ip.com:80/?type=ws&encryption=none&host=Shuposipet.com&path=%2Fde%3Fed%3D1048#[test]@test";
        let data = get_vless_data(v);
        assert_eq!(data.address_data.address, "nwarne.fast-ip.com");
        assert_eq!(data.address_data.uuid, "2dc56709-sdfs-sdfs-2234-128904");
        assert_eq!(data.address_data.port, 80);
        assert_eq!(data.query.encryption.unwrap(), "none");
        assert_eq!(data.query.r#type.unwrap(), "ws");
    }

    #[test]
    fn vless_test() {
        let v = "vless://4d2c3e35-749d-52e3-bdb6-3f3f4950c183@tre.test.one:2053?security=reality&type=tcp&flow=xtls-rprx-vision#test-name";
        let data = get_vless_data(v);
        assert_eq!(data.address_data.address, "tre.test.one");
        assert_eq!(
            data.address_data.uuid,
            "4d2c3e35-749d-52e3-bdb6-3f3f4950c183"
        );
        assert_eq!(data.address_data.port, 2053);
        assert_eq!(data.query.flow.unwrap(), "xtls-rprx-vision");
        assert_eq!(data.query.security.unwrap(), "reality");
        assert_eq!(data.query.r#type.unwrap(), "tcp");
    }

    #[test]
    fn parse_vless_query_data() {
        let query = "security=reality&sni=bench.sh&fp=chrome&pbk=7xhH4b_VkliBxGulljcyPOH-bYUA2dl-XAdZAsfhk04&sid=6ba85179e30d4fc2&type=tcp&flow=xtls-rprx-vision&alpn=http%2F1.1&path=/";
        let parsed_query = parse_vless_query(query);
        assert_eq!(parsed_query.sni.unwrap(), "bench.sh");
        assert_eq!(parsed_query.security.unwrap(), "reality");
        assert_eq!(parsed_query.fp.unwrap(), "chrome");
        assert_eq!(
            parsed_query.pbk.unwrap(),
            "7xhH4b_VkliBxGulljcyPOH-bYUA2dl-XAdZAsfhk04"
        );
        assert_eq!(parsed_query.sid.unwrap(), "6ba85179e30d4fc2");
        assert_eq!(parsed_query.r#type.unwrap(), "tcp");
        assert_eq!(parsed_query.flow.unwrap(), "xtls-rprx-vision");
        assert_eq!(parsed_query.alpn.unwrap(), "http/1.1");
        assert_eq!(parsed_query.path.unwrap(), "/");
        assert_eq!(parsed_query.encryption, None);
        assert_eq!(parsed_query.header_type, None);
        assert_eq!(parsed_query.host, None);
        assert_eq!(parsed_query.seed, None);
        assert_eq!(parsed_query.quic_security, None);
        assert_eq!(parsed_query.key, None);
        assert_eq!(parsed_query.mode, None);
        assert_eq!(parsed_query.service_name, None);
        assert_eq!(parsed_query.slpn, None);
        assert_eq!(parsed_query.spx, None);
    }

    #[test]
    fn parse_vless_query_with_defaults() {
        let query = "";
        let parsed_query = parse_vless_query(query);
        assert_eq!(parsed_query.sni, None);
        assert_eq!(parsed_query.security, None);
        assert_eq!(parsed_query.fp, None);
        assert_eq!(parsed_query.pbk, None);
        assert_eq!(parsed_query.sid, None);
        assert_eq!(parsed_query.r#type, None);
        assert_eq!(parsed_query.r#flow, None);
        assert_eq!(parsed_query.path, None);
        assert_eq!(parsed_query.encryption, None);
        assert_eq!(parsed_query.header_type, None);
        assert_eq!(parsed_query.host, None);
        assert_eq!(parsed_query.seed, None);
        assert_eq!(parsed_query.quic_security, None);
        assert_eq!(parsed_query.key, None);
        assert_eq!(parsed_query.mode, None);
        assert_eq!(parsed_query.service_name, None);
        assert_eq!(parsed_query.slpn, None);
        assert_eq!(parsed_query.spx, None);
    }

    #[test]
    fn parse_vless_host() {
        let raw_host = "uu0id@127.0.0.1:3012";
        let parsed = parse_vless_address(raw_host);
        assert_eq!(parsed.address, "127.0.0.1");
        assert_eq!(parsed.port, 3012);
        assert_eq!(parsed.uuid, "uu0id");
    }

    #[test]
    fn parse_vless_ipv6_host() {
        let raw_host = "uu0id@[2a06:98c1:3120::1]:443";
        let parsed = parse_vless_address(raw_host);
        assert_eq!(parsed.port, 443);
        assert_eq!(parsed.address, "[2a06:98c1:3120::1]");
        assert_eq!(parsed.uuid, "uu0id");
    }
}
