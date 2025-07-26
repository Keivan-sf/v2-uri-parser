use crate::config_models;

pub struct InboundGenerationOptions {
    pub socks_port: Option<u16>,
    pub http_port: Option<u16>,
}

pub fn generate_inbound_config(options: InboundGenerationOptions) -> Vec<config_models::Inbound> {
    let mut inbounds: Vec<config_models::Inbound> = vec![];
    match options.socks_port {
        Some(port) => {
            inbounds.push(generate_socks_inbound(port));
        }
        None => {}
    }

    match options.http_port {
        Some(port) => {
            inbounds.push(generate_http_inbound(port));
        }
        None => {}
    }

    return inbounds;
}

pub fn generate_http_inbound(http_port: u16) -> config_models::Inbound {
    return config_models::Inbound {
        protocol: String::from("http"),
        port: http_port,
        tag: String::from("http-in"),
        settings: None,
        listen: String::from("127.0.0.1"),
        sniffing: Some(config_models::SniffingSettings {
            enabled: Some(true),
            routeOnly: Some(true),
            metadataOnly: Some(false),
            domainsExcluded: None,
            destOverride: Some(vec![
                String::from("http"),
                String::from("tls"),
                String::from("quic"),
            ]),
        }),
    };
}

pub fn generate_socks_inbound(socks_port: u16) -> config_models::Inbound {
    return config_models::Inbound {
        protocol: String::from("socks"),
        port: socks_port,
        tag: String::from("socks-in"),
        listen: String::from("127.0.0.1"),
        settings: Some(config_models::InboundSettings { udp: true }),
        sniffing: Some(config_models::SniffingSettings {
            enabled: Some(true),
            routeOnly: Some(true),
            metadataOnly: Some(false),
            domainsExcluded: None,
            destOverride: Some(vec![
                String::from("http"),
                String::from("tls"),
                String::from("quic"),
            ]),
        }),
    };
}
