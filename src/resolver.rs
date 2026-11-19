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
    lookup_ip::LookupIp,
};

// Get Config struct from cli_manager
use crate::cli::Config;

pub fn resolve(mut lookup_config: Config) -> String {

    // Set default server configuration
    // Will change to to changable by the user but not yet
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

    // Append a dot to the end of the address to tell it to search harder
    if lookup_config.depth == false {
        lookup_config.query.push_str(".");
    }

    let io_loop: Runtime = Runtime::new().unwrap();

    // Tell the resolver how to make the lookup and what methods to use
    let resolver: Resolver<TokioRuntimeProvider> = Resolver::builder_with_config(
        ResolverConfig::udp_and_tcp(&dns_server),
        TokioRuntimeProvider::default()
    ).build().unwrap();

    // Look at if the user wants ipv4 or ipv6, i know this is janky

    let address = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        get_address(lookup_config, resolver, io_loop)
    })).unwrap_or_else(|_| "Unable to find assosiated ip address".to_string());

    // We return the final adresses back to the main function
    address
}

fn get_address(
    lookup_config: Config,
    resolver: Resolver<TokioRuntimeProvider>,
    io_loop: Runtime,) -> String {

    let response = if lookup_config.ip_ver == 6 {
        io_loop.block_on(resolver.ipv6_lookup(lookup_config.query))
    } else {
        io_loop.block_on(resolver.ipv4_lookup(lookup_config.query))
    };

    let address = match response {
        Ok(response) => {
            let response: LookupIp = response.into();
            response.iter().next()
        }
        Err(_) => None,
    };

    match address {
        Some(address) => address.to_string(),
        None => "Unable to find associated IP address".to_string(),
    }
}

