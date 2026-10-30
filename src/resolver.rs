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

pub fn resolve(lookup_query: &str) -> IpAddr {

    let dns_server: ServerGroup<'_> = ServerGroup {
        ips: &vec![
            IpAddr::V4(Ipv4Addr::new(1, 1, 1, 1)),
            IpAddr::V4(Ipv4Addr::new(1, 0, 0, 1)),
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

    let lookup_future = resolver.lookup_ip(lookup_query);

    let response = io_loop.block_on(lookup_future);

    // Check if an error is given from the DNS server
    let checked_response: hickory_resolver::lookup_ip::LookupIp = match response {
        Ok(resp) => resp,                     // success – use the value
        Err(_e) => {
            println!("no address returned! '{lookup_query}' did not return a valid address");
            return IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0));
        }
    };
    
    let address: IpAddr = checked_response.iter().next().expect("no address returned!");

    return address;
}
