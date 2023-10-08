pub enum protocols {
    Vmess,
    Vless,
    Shadowsocks,
    Trojan,
    Socks,
    Http,
}

pub fn get_uri_format(uri: &str) -> Option<protocols> {
    if uri.starts_with("vmess://") {
        return Some(protocols::Vmess);
    }
    if uri.starts_with("vless://") {
        return Some(protocols::Vless);
    }
    return None;
}
