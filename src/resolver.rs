use tokio::runtime::Runtime;
use std::net::{
    IpAddr,
    Ipv4Addr,
    Ipv6Addr,
};
use hickory_resolver::{
    Resolver,
    net::runtime::TokioRuntimeProvider,
    config::ResolverConfig,
    config::ServerGroup
};

pub fn resolve() {

    let dns_server = ServerGroup {
        ips: &vec![
            IpAddr::V4(Ipv4Addr::new(1, 1, 1, 1)),
            IpAddr::V4(Ipv4Addr::new(1, 0, 0, 1)),
            IpAddr::V6(Ipv6Addr::new(0x2606, 0x4700, 0x4700, 0, 0, 0, 0, 0x1111)),
            IpAddr::V6(Ipv6Addr::new(0x2606, 0x4700, 0x4700, 0, 0, 0, 0, 0x1001)),
        ],
        server_name: &"cloudflare-dns.com".to_string(),
        path: &"/dns-query".to_string(),
    };

    let url: &str = "google.com";

    let io_loop: Runtime = Runtime::new().unwrap();

    let resolver: Resolver<TokioRuntimeProvider> = Resolver::builder_with_config(
    ResolverConfig::udp_and_tcp(&dns_server),
    TokioRuntimeProvider::default()
    ).build().unwrap();

    let lookup_future = resolver.lookup_ip(url);
    let response: hickory_resolver::lookup_ip::LookupIp = io_loop.block_on(lookup_future).unwrap();
    let address: IpAddr = response.iter().next().expect("no addresses returned!");

    println!("{address}")
}
