use querystring;
mod models;
use crate::config_models::*;
use http::Uri;
use std::process::exit;

pub fn create_outbound_object(data: models::VlessData) -> Outbound {
    return Outbound {
        protocol: String::from("vless"),
        tag: String::from("proxy"),
        streamSettings: StreamSettings {
            network: data.query.r#type.clone(),
            security: data.query.security.clone(),
            tlsSettings: if data.query.security == String::from("tls") {
                Some(TlsSettings {
                    rejectUnknownSni: None,
                    enableSessionResumption: None,
                    minVersion: None,
                    maxVersion: None,
                    cipherSuites: None,
                    disableSystemRoot: None,
                    preferServerCipherSuites: None,
                    fingerprint: Some(data.query.fp.clone()),
                    serverName: Some(data.query.sni.clone()),
                    allowInsecure: Some(false),
                })
            } else {
                None
            },
            wsSettings: if data.query.r#type == String::from("ws") {
                Some(WsSettings {
                    headers: Some(HeaderSetting {
                        Host: Some(data.query.host),
                    }),
                    path: Some(
                        urlencoding::decode(data.query.path.as_str())
                            .unwrap()
                            .into_owned(),
                    ),
                    acceptProxyProtocol: None,
                })
            } else {
                None
            },
            tcpSettings: if data.query.r#type == String::from("tcp") {
                Some(TCPSettings {
                    header: Some(TCPHeader {
                        r#type: data.query.header_type.clone(),
                    }),
                    acceptProxyProtocol: None,
                })
            } else {
                None
            },
            realitySettings: if data.query.security == String::from("reality") {
                Some(RealitySettings {
                    publicKey: data.query.pbk,
                    serverName: data.query.sni.clone(),
                    shortId: data.query.sid,
                    spiderX: String::from(""),
                    fingerprint: data.query.fp,
                })
            } else {
                None
            },
            grpcSettings: if data.query.r#type == String::from("grpc") {
                Some(GRPCSettings {
                    multiMode: false,
                    serviceName: data.query.service_name,
                })
            } else {
                None
            },
            quicSettings: if data.query.r#type == String::from("quic") {
                Some(QuicSettings {
                    header: Some(NonHeaderObject {
                        r#type: String::from("none"),
                    }),
                    security: String::from("none"),
                    key: String::from(""),
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
                    encryption: if data.query.encryption.len() > 0 {
                        data.query.encryption
                    } else {
                        String::from("none")
                    },
                    level: 0,
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
        uuid,
        address: parsed.host().unwrap().to_string(),
        port: parsed.port().unwrap().as_u16(),
    };
}

fn parse_vless_query(raw_query: &str) -> models::VlessQuery {
    let query: Vec<(&str, &str)> = querystring::querify(raw_query);

    let a = models::VlessQuery {
        path: get_parameter_value(&query, "path"),
        pbk: get_parameter_value(&query, "pbk"),
        security: get_parameter_value(&query, "security"),
        sid: get_parameter_value(&query, "sid"),
        flow: get_parameter_value(&query, "flow"),
        sni: get_parameter_value(&query, "sni"),
        fp: get_parameter_value(&query, "fp"),
        r#type: get_parameter_value(&query, "type"),
        encryption: get_parameter_value(&query, "encryption"),
        header_type: get_parameter_value(&query, "headerType"),
        host: get_parameter_value(&query, "host"),
        seed: get_parameter_value(&query, "seed"),
        quic_security: get_parameter_value(&query, "quicSecurity"),
        key: get_parameter_value(&query, "key"),
        mode: get_parameter_value(&query, "mode"),
        service_name: get_parameter_value(&query, "serviceName"),
        slpn: get_parameter_value(&query, "slpn"),
        spx: get_parameter_value(&query, "spx"),
    };
    return a;
}

fn get_parameter_value(query: &Vec<(&str, &str)>, param: &str) -> String {
    return query
        .iter()
        .find(|q| String::from(q.0) == String::from(param))
        .unwrap_or(&("", ""))
        .1
        .to_string();
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
            "http"
        )
    }

    #[test]
    fn vless_test_2() {
        let v = "vless://2dc56709-sdfs-sdfs-2234-128904@nwarne.fast-ip.com:80/?type=ws&encryption=none&host=Shuposipet.com&path=%2Fde%3Fed%3D1048#[test]@test";
        let data = get_vless_data(v);
        assert_eq!(data.address_data.address, "nwarne.fast-ip.com");
        assert_eq!(data.address_data.uuid, "2dc56709-sdfs-sdfs-2234-128904");
        assert_eq!(data.address_data.port, 80);
        assert_eq!(data.query.encryption, "none");
        assert_eq!(data.query.r#type, "ws");
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
        assert_eq!(data.query.flow, "xtls-rprx-vision");
        assert_eq!(data.query.security, "reality");
        assert_eq!(data.query.r#type, "tcp");
    }

    #[test]
    fn parse_vless_query_data() {
        let query = "security=reality&sni=bench.sh&fp=chrome&pbk=7xhH4b_VkliBxGulljcyPOH-bYUA2dl-XAdZAsfhk04&sid=6ba85179e30d4fc2&type=tcp&flow=xtls-rprx-vision&path=/";
        let parsed_query = parse_vless_query(query);
        assert_eq!(parsed_query.sni, "bench.sh");
        assert_eq!(parsed_query.security, "reality");
        assert_eq!(parsed_query.fp, "chrome");
        assert_eq!(
            parsed_query.pbk,
            "7xhH4b_VkliBxGulljcyPOH-bYUA2dl-XAdZAsfhk04"
        );
        assert_eq!(parsed_query.sid, "6ba85179e30d4fc2");
        assert_eq!(parsed_query.r#type, "tcp");
        assert_eq!(parsed_query.flow, "xtls-rprx-vision");
        assert_eq!(parsed_query.path, "/");
        assert_eq!(parsed_query.encryption, "");
        assert_eq!(parsed_query.header_type, "");
        assert_eq!(parsed_query.host, "");
        assert_eq!(parsed_query.seed, "");
        assert_eq!(parsed_query.quic_security, "");
        assert_eq!(parsed_query.key, "");
        assert_eq!(parsed_query.mode, "");
        assert_eq!(parsed_query.service_name, "");
        assert_eq!(parsed_query.slpn, "");
        assert_eq!(parsed_query.spx, "");
    }

    #[test]
    fn parse_vless_query_with_defaults() {
        let query = "";
        let parsed_query = parse_vless_query(query);
        assert_eq!(parsed_query.sni, "");
        assert_eq!(parsed_query.security, "");
        assert_eq!(parsed_query.fp, "");
        assert_eq!(parsed_query.pbk, "");
        assert_eq!(parsed_query.sid, "");
        assert_eq!(parsed_query.r#type, "");
        assert_eq!(parsed_query.r#flow, "");
        assert_eq!(parsed_query.path, "");
        assert_eq!(parsed_query.encryption, "");
        assert_eq!(parsed_query.header_type, "");
        assert_eq!(parsed_query.host, "");
        assert_eq!(parsed_query.seed, "");
        assert_eq!(parsed_query.quic_security, "");
        assert_eq!(parsed_query.key, "");
        assert_eq!(parsed_query.mode, "");
        assert_eq!(parsed_query.service_name, "");
        assert_eq!(parsed_query.slpn, "");
        assert_eq!(parsed_query.spx, "");
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
        let v = "vless://4d91916f-a7fd-419b-8b90-640bb8d1b9f4@[2a06:98c1:3120::1]:443?path=%2FPSZPkYG71g6bn84o%2FMTQxLjE0OC4yMDMuNg&security=tls&alpn=http%2F1.1&encryption=none&host=titantablomanahamrah.ir&fp=randomized&type=ws&sni=TITanTabLOmaNAHaMRaH.IR#";
        let raw_host = "uu0id@[2a06:98c1:3120::1]:443";
        let parsed = parse_vless_address(raw_host);
        assert_eq!(parsed.port, 443);
        assert_eq!(parsed.address, "[2a06:98c1:3120::1]");
        assert_eq!(parsed.uuid, "uu0id");
    }

    #[test]
    fn create_outbound_for_tcp_reality() {
        let v = "vless://3d2c2r05-y739-51e3-bd86-3f3f4950c183@tr.deet23ngdell.com:1818?security=reality&encryption=none&pbk=7xhH8b_VkliBxgulljcyPOH-bYoA2dl-XAdZAsfhk04&headerType=none&fp=chrome&type=tcp&flow=xtls-rprx-vision&sni=bench.sh&sid=6bt85979e30d4fc2#%F0%9F%87%B9%F0%9F%87%B7+H";
        let data = get_vless_data(v);
        let outbound_object = create_outbound_object(data);
        let serialized = serde_json::to_string(&outbound_object).unwrap();

        assert_eq!(
            serialized,
            r#"{"settings":{"vnext":[{"address":"tr.deet23ngdell.com","port":1818,"users":[{"id":"3d2c2r05-y739-51e3-bd86-3f3f4950c183","encryption":"none","flow":"xtls-rprx-vision","level":0}]}]},"streamSettings":{"network":"tcp","security":"reality","tlsSettings":null,"wsSettings":null,"tcpSettings":{"header":{"type":"none"},"acceptProxyProtocol":null},"realitySettings":{"fingerprint":"chrome","serverName":"bench.sh","publicKey":"7xhH8b_VkliBxgulljcyPOH-bYoA2dl-XAdZAsfhk04","shortId":"6bt85979e30d4fc2","spiderX":""},"grpcSettings":null,"quicSettings":null},"protocol":"vless","tag":"proxy"}"#
        );
    }
}
