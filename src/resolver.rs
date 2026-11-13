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
    config::ServerGroup,
};

// Get Config struct from cli_manager
use crate::cli_manager::Config;

pub fn resolve(lookup_config: Config) -> IpAddr {

    let dns_server: ServerGroup<'_> = ServerGroup {
        ips: &vec![
            IpAddr::V4(Ipv4Addr::new(1, 1, 1, 1)),  // Cloudflare Primary v4
            IpAddr::V4(Ipv4Addr::new(1, 0, 0, 1)),  // Cloudflare Secondary v4
            IpAddr::V6(Ipv6Addr::new(0x2606, 0x4700, 0x4700, 0, 0, 0, 0, 0x1111)),
            IpAddr::V6(Ipv6Addr::new(0x2606, 0x4700, 0x4700, 0, 0, 0, 0, 0x1001)),
        ],
        server_name: &"cloudflare-dns.com".to_string(),
        path: &"/dns-query".to_string(),
    };

    let io_loop: Runtime = Runtime::new().unwrap();

    let resolver: Resolver<TokioRuntimeProvider> = Resolver::builder_with_config(
    ResolverConfig::udp_and_tcp(&dns_server),
    TokioRuntimeProvider::default()
    ).build().unwrap();

    let lookup_future = resolver.ipv4_lookup(lookup_config.query);

    let response: hickory_resolver::lookup_ip::LookupIp = io_loop.block_on(lookup_future).unwrap().into();

    let address: IpAddr = response.iter().next().expect("no addresses returned!");

    // We return the final adresses back to the main function
    address
}

