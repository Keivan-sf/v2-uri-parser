# v2-uri-parser

V2ray URI parser for xray core

Currently supports: `vless`, `vmess`, `shadowsocks`, `trojan` and `socks`

```
Parses V2ray URI and generates JSON config for xray

Usage: v2parser [OPTIONS] <uri>

Arguments:
  <uri>  V2ray URI to parse

Options:
      --socksport <PORT>  Optional SOCKS5 proxy port for inbound
      --httpport <PORT>   Optional HTTP proxy port for inbound
      --get-name          Only print the config name
  -h, --help              Print help
  -V, --version           Print version
```
