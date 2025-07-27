use regex::Regex;

pub enum Protocols {
    Vmess,
    Vless,
    Shadowsocks,
    Trojan,
    Socks,
    Http,
}

pub fn get_uri_protocol(uri: &str) -> Option<Protocols> {
    if uri.starts_with("vmess://") {
        return Some(Protocols::Vmess);
    }
    if uri.starts_with("vless://") {
        return Some(Protocols::Vless);
    }
    if uri.starts_with("ss://") {
        return Some(Protocols::Shadowsocks);
    }
    if uri.starts_with("socks5://") || uri.starts_with("socks4://") || uri.starts_with("socks://") {
        return Some(Protocols::Socks);
    }
    if uri.starts_with("http://") {
        return Some(Protocols::Http);
    }
    if uri.starts_with("trojan://") {
        return Some(Protocols::Trojan);
    }
    return None;
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn return_none_for_invalid_uri() {
        let protocol = get_uri_protocol("123-vless://3d1c3f04-729d-59d3-bdb6-3f3f4352e173@root.ii.one:2083?security=reality&sni=www.spamhaus.org&fp=safari&pbk=7xhH4b_VkliBxGulljcyPOH-bYUA2dl-XAdZAsfhk04&sid=6ba85179e30d4fc2&type=tcp&flow=xtls-rprx-vision#Ha-ac");
        assert!(matches!(protocol, None));
    }
    #[test]
    fn recognize_vless_protocol() {
        let protocol = get_uri_protocol("vless://3d1c3f04-729d-59d3-bdb6-3f3f4352e173@root.ii.one:2083?security=reality&sni=www.spamhaus.org&fp=safari&pbk=7xhH4b_VkliBxGulljcyPOH-bYUA2dl-XAdZAsfhk04&sid=6ba85179e30d4fc2&type=tcp&flow=xtls-rprx-vision#Ha-ac").unwrap();
        assert!(matches!(protocol, Protocols::Vless));
    }
    #[test]
    fn recognize_vmess_protocol() {
        let protocol = get_uri_protocol("vmess://eyJhZGQiOiIxMjcuMC4wLjEiLCJhaWQiOiIwIiwiaG9zdCI6IiIsImlkIjoiOHM2OTdlMmMtZXMxNy00MDNkLTI0ZjMtZHMyYzYwc2I4ZjUiLCJuZXQiOiJ0Y3AiLCJwYXRoIjoiIiwicG9ydCI6IjgwODAiLCJwcyI6InRlc3QiLCJzY3kiOiJhdXRvIiwic25pIjoiIiwidGxzIjoiIiwidHlwZSI6Im5vbmUiLCJ2IjoiMiJ9").unwrap();
        assert!(matches!(protocol, Protocols::Vmess));
    }
    #[test]
    fn recognize_shadowsocks_protocol() {
        let protocol = get_uri_protocol("ss://Y2hhY2hhMjAtaWV0Zi1wb2x5MTMwNTpXNzRYRkFMS0t1dzZtNUlB@www.outline.aasf.cyou:443#test").unwrap();
        assert!(matches!(protocol, Protocols::Shadowsocks));
    }
    #[test]
    fn recognize_trojan_protocol() {
        let protocol = get_uri_protocol("trojan://test-pw@13.50.100.84:22222?security=tls&sni=trj.rollingnext.co.uk&type=tcp#test").unwrap();
        assert!(matches!(protocol, Protocols::Trojan));
    }
}
