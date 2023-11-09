use crate::config_models;

pub struct InboundGenerationOptions {
    pub socks_port: Option<u16>,
}

pub fn generate_inbound_config(options: InboundGenerationOptions) -> Vec<config_models::Inbound> {
    let mut inbounds: Vec<config_models::Inbound> = vec![];
    match options.socks_port {
        Some(port) => {
            inbounds.push(generate_socks_inbound(port));
        }
        None => {}
    }
    return inbounds;
}

pub fn generate_socks_inbound(socks_port: u16) -> config_models::Inbound {
    return config_models::Inbound {
        protocol: String::from("socks"),
        port: socks_port,
        tag: String::from("socks-in"),
        listen: String::from("127.0.0.1"),
        settings: config_models::InboundSettings { udp: true },
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

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_generate_socks_inboud() {
        let socks_inbound = generate_socks_inbound(2080);
        assert_eq!(socks_inbound.protocol, String::from("socks"));
        assert_eq!(socks_inbound.listen, String::from("127.0.0.1"));
        assert_eq!(socks_inbound.tag, String::from("socks-in"));
        assert_eq!(socks_inbound.port, 2080);
        assert_eq!(socks_inbound.settings.udp, true);

        let sniffing_obj = socks_inbound.sniffing.unwrap();
        assert_eq!(sniffing_obj.enabled, Some(true));
        assert_eq!(sniffing_obj.routeOnly, Some(true));
        assert_eq!(sniffing_obj.domainsExcluded, None);
        assert_eq!(
            sniffing_obj.destOverride,
            Some(vec![
                String::from("http"),
                String::from("tls"),
                String::from("quic"),
            ])
        );
    }
}
