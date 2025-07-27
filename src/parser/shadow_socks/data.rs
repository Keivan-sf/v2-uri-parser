use http::Uri;

use crate::{
    config_models::RawData,
    parser::shadow_socks::models,
    utils::{url_decode, url_decode_str},
};
use base64::{engine::general_purpose, Engine};

pub fn get_data(uri: &str) -> RawData {
    let data = uri.split_once("ss://").unwrap().1;
    let (raw_data, name) = data.split_once("#").unwrap_or((data, ""));
    let (raw_uri, _) = raw_data.split_once("?").unwrap_or((raw_data, ""));
    let parsed_address = parse_ss_address(raw_uri);
    return RawData {
        remarks: url_decode(Some(String::from(name))).unwrap_or(String::from("")),
        server_method: url_decode(Some(parsed_address.method)),
        address: Some(parsed_address.address),
        port: Some(parsed_address.port),
        uuid: url_decode(Some(parsed_address.password)),
        r#type: Some(String::from("tcp")),
        header_type: Some(String::from("none")),
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

fn parse_ss_address(raw_data: &str) -> models::ShadowSocksAddress {
    let (userinfo, raw_address): (String, &str) = match raw_data.split_once("@") {
        None => {
            panic!("Wrong shadowsocks format, no `@` found in the address");
        }
        Some(data) => (String::from(data.0), data.1),
    };
    let address_wo_slash = raw_address.strip_suffix("/").unwrap_or(raw_address);

    let parsed = address_wo_slash.parse::<Uri>().unwrap();

    let a = general_purpose::STANDARD
        .decode(url_decode_str(&userinfo).unwrap_or(userinfo))
        .expect("User info is not base64");

    let (method, password) = std::str::from_utf8(&a)
        .expect("Base64 did not yield a valid utf-8 string")
        .split_once(":")
        .expect("No `:` found in the decoded base64");

    return models::ShadowSocksAddress {
        method: String::from(method),
        password: String::from(password),
        address: parsed.host().unwrap().to_string(),
        port: parsed.port().unwrap().as_u16(),
    };
}
