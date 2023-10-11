use std::process::exit;

use querystring;

#[derive(PartialEq, Eq)]
pub struct VlessQuery {
    security: String,
    sni: String,
    fp: String,
    pbk: String,
    sid: String,
    r#type: String,
    flow: String,
    path: String,
    encryption: String,
    header_type: String,
    host: String,
    seed: String,
    quic_security: String,
    r#key: String,
    mode: String,
    service_name: String,
    slpn: String,
    spx: String,
}

pub struct VlessAddress {
    uuid: String,
    address: String,
    port: u16,
}

pub fn get_vless_data(uri: &str) {
    let data = uri.split_once("vless://").unwrap().1;
    let query_and_name = uri.split_once("?").unwrap().1;
    let query = query_and_name
        .split_once("#")
        .unwrap_or((query_and_name, ""))
        .0;
    let parsed_query = parse_vless_query(query);
    println!("{0}", parsed_query.flow);
}

pub fn parse_vless_address(raw_data: &str) -> VlessAddress {
    let (uuid, raw_address): (String, &str) = match raw_data.split_once("@") {
        None => {
            println!("Wrong vless format, no `@` found in the address");
            exit(0);
        }
        Some(data) => (String::from(data.0), data.1),
    };
    let (address, port): (String, u16) = match raw_address.split_once(":") {
        None => {
            println!("Wrong vless format, no `:` found in the address");
            exit(0);
        }
        Some(data) => (
            String::from(data.0),
            data.1
                .parse::<u16>()
                .expect("Wrong vless format, port is not a number"),
        ),
    };
    return VlessAddress {
        uuid,
        address,
        port,
    };
}

pub fn parse_vless_query(raw_query: &str) -> VlessQuery {
    let query: Vec<(&str, &str)> = querystring::querify(raw_query);

    let a = VlessQuery {
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
    fn vless_test() {
        let v = "vless://4d2c3e35-749d-52e3-bdb6-3f3f4950c183@tre.test.one:2053?security=reality&sni=bench.sh&fp=chrome&pbk=7xhH4b_VkliBxGulljcyPOH-bYUA2dl-XAdZAsfhk04&sid=6ba85179e30d4fc2&type=tcp&flow=xtls-rprx-vision#test-name";
        get_vless_data(v);
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
}
