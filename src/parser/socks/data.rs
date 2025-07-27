use http::Uri;

use crate::{
    config_models::RawData,
    parser::socks::models,
    utils::{url_decode, url_decode_str},
};
use base64::{engine::general_purpose, Engine};

pub fn get_data(uri: &str) -> RawData {
    let data = uri.split_once("://").unwrap().1;
    let (raw_data, name) = data.split_once("#").unwrap_or((data, ""));
    let (raw_uri, _) = raw_data.split_once("?").unwrap_or((raw_data, ""));
    let parsed_address = parse_socks_address(raw_uri);
    return RawData {
        remarks: url_decode(Some(String::from(name))).unwrap_or(String::from("")),
        username: url_decode(parsed_address.username),
        address: Some(parsed_address.address),
        port: Some(parsed_address.port),
        uuid: url_decode(parsed_address.password),
        r#type: Some(String::from("tcp")),
        header_type: None,
        server_method: None,
        security: None,
        fp: None,
        sni: None,
        pbk: None,
        sid: None,
        key: None,
        spx: None,
        flow: None,
        path: None,
        host: None,
        seed: None,
        mode: None,
        slpn: None,
        alpn: None,
        extra: None,
        authority: None,
        encryption: None,
        service_name: None,
        quic_security: None,
        allowInsecure: None,
        vnext_security: None,
    };
}

fn parse_socks_address(raw_data: &str) -> models::SocksAddress {
    let (maybe_userinfo, raw_address): (Option<String>, &str) = match raw_data.split_once("@") {
        Some(data) => (Some(String::from(data.0)), data.1),
        None => (None, raw_data),
    };
    let address_wo_slash = raw_address.strip_suffix("/").unwrap_or(raw_address);

    let parsed = address_wo_slash.parse::<Uri>().unwrap();

    return match maybe_userinfo {
        Some(userinfo) => {
            let url_decoded = url_decode_str(&userinfo).unwrap_or(userinfo);
            let username_and_password = general_purpose::STANDARD
                .decode(url_decoded.clone())
                .map(|a| {
                    String::from(
                        std::str::from_utf8(&a).expect("Base64 did not yield a valid utf-8 string"),
                    )
                })
                .unwrap_or(String::from(url_decoded.clone()));

            let (username, password) = username_and_password
                .split_once(":")
                .expect("No `:` found in the decoded base64");

            models::SocksAddress {
                username: Some(String::from(username)),
                password: Some(String::from(password)),
                address: parsed.host().unwrap().to_string(),
                port: parsed.port().unwrap().as_u16(),
            }
        }
        None => models::SocksAddress {
            username: None,
            password: None,
            address: parsed.host().unwrap().to_string(),
            port: parsed.port().unwrap().as_u16(),
        },
    };
}
