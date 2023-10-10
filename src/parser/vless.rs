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
}

pub fn get_vless_data(uri: &str) {
    let data = uri.split_once("vless://").unwrap().1;
    let query = uri.split_once("?").unwrap().1;
    let query_parsed = querystring::querify(query);
    for i in query_parsed {
        println!("{0} : {1}", i.0, i.1);
    }
}

pub fn get_vless_query_data(raw_query: &str) -> VlessQuery {
    let query = querystring::querify(raw_query);
    let a = VlessQuery {
        path: query
            .iter()
            .find(|q| String::from(q.0) == String::from("path"))
            .unwrap_or(&("", ""))
            .1
            .to_string(),
        pbk: query
            .iter()
            .find(|q| String::from(q.0) == String::from("pbk"))
            .unwrap_or(&("", ""))
            .1
            .to_string(),
        security: query
            .iter()
            .find(|q| String::from(q.0) == String::from("security"))
            .unwrap_or(&("", ""))
            .1
            .to_string(),
        sid: query
            .iter()
            .find(|q| String::from(q.0) == String::from("sid"))
            .unwrap_or(&("", ""))
            .1
            .to_string(),
        flow: query
            .iter()
            .find(|q| String::from(q.0) == String::from("flow"))
            .unwrap_or(&("", ""))
            .1
            .to_string(),
        sni: query
            .iter()
            .find(|q| String::from(q.0) == String::from("sni"))
            .unwrap_or(&("", ""))
            .1
            .to_string(),
        fp: query
            .iter()
            .find(|q| String::from(q.0) == String::from("fp"))
            .unwrap_or(&("", ""))
            .1
            .to_string(),
        r#type: query
            .iter()
            .find(|q| String::from(q.0) == String::from("type"))
            .unwrap_or(&("", ""))
            .1
            .to_string(),
    };
    return a;
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
    fn parse_vless_query() {
        let query = "security=reality&sni=bench.sh&fp=chrome&pbk=7xhH4b_VkliBxGulljcyPOH-bYUA2dl-XAdZAsfhk04&sid=6ba85179e30d4fc2&type=tcp&flow=xtls-rprx-vision&path=/";
        let parsed_query = get_vless_query_data(query);
        assert_eq!(parsed_query.sni, "bench.sh");
        assert_eq!(parsed_query.security, "reality");
        assert_eq!(parsed_query.fp, "chrome");
        assert_eq!(
            parsed_query.pbk,
            "7xhH4b_VkliBxGulljcyPOH-bYUA2dl-XAdZAsfhk04"
        );
        assert_eq!(parsed_query.sid, "6ba85179e30d4fc2");
        assert_eq!(parsed_query.r#type, "tcp");
        assert_eq!(parsed_query.r#flow, "xtls-rprx-vision");
        assert_eq!(parsed_query.path, "/");
    }
    #[test]
    fn parse_vless_query_with_defaults() {
        let query = "";
        let parsed_query = get_vless_query_data(query);
        assert_eq!(parsed_query.sni, "");
        assert_eq!(parsed_query.security, "");
        assert_eq!(parsed_query.fp, "");
        assert_eq!(parsed_query.pbk, "");
        assert_eq!(parsed_query.sid, "");
        assert_eq!(parsed_query.r#type, "");
        assert_eq!(parsed_query.r#flow, "");
        assert_eq!(parsed_query.path, "");
    }
}
