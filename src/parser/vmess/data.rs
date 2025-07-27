use crate::config_models::RawData;
use crate::parser::vmess::models::{self, VmessAddress};
use crate::utils::{get_parameter_value, url_decode};
use base64::{engine::general_purpose, Engine};
use http::Uri;
use serde_json::Value;

pub fn get_data(uri: &str) -> RawData {
    let data = uri.split_once("vmess://").unwrap().1;

    return match general_purpose::STANDARD.decode(data) {
        Ok(decoded) => get_raw_data_from_base64(&decoded),
        Err(_) => get_raw_data_from_uri(data),
    };
}

fn get_raw_data_from_base64(decoded_base64: &Vec<u8>) -> RawData {
    let json_str = std::str::from_utf8(decoded_base64).unwrap();
    let json = serde_json::from_str::<Value>(json_str).unwrap();

    return RawData {
        remarks: get_str_field(&json, "ps").unwrap_or(String::from("")),
        uuid: get_str_field(&json, "id"),
        port: get_str_field(&json, "port")
            .and_then(|s| Some(s.parse::<u16>().expect("port is not a number"))),
        address: get_str_field(&json, "add"),
        alpn: url_decode(get_str_field(&json, "alpn")),
        path: url_decode(get_str_field(&json, "path")),
        authority: url_decode(get_str_field(&json, "host")),
        // this probably does not exist in vmess uri
        pbk: url_decode(get_str_field(&json, "pbk")),
        security: get_str_field(&json, "tls"),
        vnext_security: get_str_field(&json, "scy"),
        // this probably does not exist in vmess uri
        sid: url_decode(get_str_field(&json, "pbk")),
        // this probably does not exist in vmess uri
        flow: url_decode(get_str_field(&json, "flow")),
        sni: get_str_field(&json, "sni"),
        fp: url_decode(get_str_field(&json, "fp")),
        r#type: url_decode(get_str_field(&json, "net")),
        encryption: None,
        header_type: url_decode(get_str_field(&json, "type")),
        host: url_decode(get_str_field(&json, "host")),
        // this probably does not exist in vmess uri
        seed: url_decode(get_str_field(&json, "seed")),
        quic_security: None,
        key: None,
        mode: url_decode(get_str_field(&json, "type")),
        service_name: url_decode(get_str_field(&json, "path")),
        // this probably does not exist in vmess uri
        slpn: url_decode(get_str_field(&json, "slpn")),
        // this probably does not exist in vmess uri
        spx: url_decode(get_str_field(&json, "spx")),
        // this probably does not exist in vmess uri
        extra: url_decode(get_str_field(&json, "extra")),
        // this probably does not exist in vmess uri
        allowInsecure: None,
    };
}

fn get_str_field(json: &Value, field: &str) -> Option<String> {
    return json.get(field).and_then(|v| v.as_str()).map(String::from);
}

fn get_raw_data_from_uri(uri: &str) -> RawData {
    let data = uri.split_once("vmess://").unwrap().1;
    let query_and_name = uri.split_once("?").unwrap().1;

    let (raw_query, name) = query_and_name
        .split_once("#")
        .unwrap_or((query_and_name, ""));
    let parsed_address = parse_vmess_address(data.split_once("?").unwrap().0);
    let query: Vec<(&str, &str)> = querystring::querify(raw_query);

    return RawData {
        remarks: String::from(name),
        uuid: Some(parsed_address.uuid),
        port: Some(parsed_address.port),
        address: Some(parsed_address.address),
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
        vnext_security: None,
        slpn: get_parameter_value(&query, "slpn"),
        spx: url_decode(get_parameter_value(&query, "spx")),
        extra: url_decode(get_parameter_value(&query, "extra")),
        allowInsecure: get_parameter_value(&query, "allowInsecure"),
    };
}

fn parse_vmess_address(raw_data: &str) -> VmessAddress {
    let (uuid, raw_address): (String, &str) = match raw_data.split_once("@") {
        None => {
            panic!("Wrong vmess format, no `@` found in the address and it was not a valid base64");
        }
        Some(data) => (String::from(data.0), data.1),
    };
    let address_wo_slash = raw_address.strip_suffix("/").unwrap_or(raw_address);

    let parsed = address_wo_slash.parse::<Uri>().unwrap();

    return VmessAddress {
        uuid: url_decode(Some(uuid)).unwrap(),
        address: parsed.host().unwrap().to_string(),
        port: parsed.port().unwrap().as_u16(),
    };
}
