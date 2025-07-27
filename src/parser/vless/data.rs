use crate::config_models::RawData;
use crate::parser::vless::models;
use crate::utils::{get_parameter_value, url_decode};
use http::Uri;

pub fn get_data(uri: &str) -> RawData {
    let data = uri.split_once("vless://").unwrap().1;
    let query_and_name = uri.split_once("?").unwrap().1;
    let (raw_query, name) = query_and_name
        .split_once("#")
        .unwrap_or((query_and_name, ""));
    let parsed_address = parse_vless_address(data.split_once("?").unwrap().0);
    let query: Vec<(&str, &str)> = querystring::querify(raw_query);

    return RawData {
        remarks: url_decode(Some(String::from(name))).unwrap_or(String::from("")),
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
        server_method: None,
    };
}

fn parse_vless_address(raw_data: &str) -> models::VlessAddress {
    let (uuid, raw_address): (String, &str) = match raw_data.split_once("@") {
        None => {
            panic!("Wrong vless format, no `@` found in the address");
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
