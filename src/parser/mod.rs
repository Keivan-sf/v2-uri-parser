use regex::Regex;

pub enum protocols {
    Vmess,
    Vless,
    Shadowsocks,
    Trojan,
    Socks,
    Http,
}

pub fn get_uri_format(uri: &str) -> Option<protocols> {
    let uri_regex = Regex::new(r"^[a-z]+:\/\/.+$").unwrap();
    if !uri_regex.is_match(uri) {
        return None;
    }
    if uri.starts_with("vmess://") {
        return Some(protocols::Vmess);
    }
    if uri.starts_with("vless://") {
        return Some(protocols::Vless);
    }
    return None;
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn return_none_for_invalid_uri() {
        let protocol = get_uri_format("123-vless://3d1c3f04-729d-59d3-bdb6-3f3f4352e173@root.ii.one:2083?security=reality&sni=www.spamhaus.org&fp=safari&pbk=7xhH4b_VkliBxGulljcyPOH-bYUA2dl-XAdZAsfhk04&sid=6ba85179e30d4fc2&type=tcp&flow=xtls-rprx-vision#Ha-ac");
        assert!(matches!(protocol, None));
    }
    #[test]
    fn recognize_vless_format() {
        let protocol = get_uri_format("vless://3d1c3f04-729d-59d3-bdb6-3f3f4352e173@root.ii.one:2083?security=reality&sni=www.spamhaus.org&fp=safari&pbk=7xhH4b_VkliBxGulljcyPOH-bYUA2dl-XAdZAsfhk04&sid=6ba85179e30d4fc2&type=tcp&flow=xtls-rprx-vision#Ha-ac").unwrap();
        assert!(matches!(protocol, protocols::Vless));
    }
}
