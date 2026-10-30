use std::net::IpAddr;
use tokio::runtime::Runtime;
use hickory_resolver::{
        Resolver,
        net::runtime::TokioRuntimeProvider,
        config::ResolverConfig,
        config::CLOUDFLARE,
};

pub fn resolve() {

        let io_loop: Runtime = Runtime::new().unwrap();

        let resolver: Resolver<TokioRuntimeProvider> = Resolver::builder_with_config(
        ResolverConfig::udp_and_tcp(&CLOUDFLARE),
        TokioRuntimeProvider::default()
        ).build().unwrap();

        println!("Here1");
        let lookup_future = resolver.lookup_ip("google.com");

        println!("Here2");
        let response: hickory_resolver::lookup_ip::LookupIp = io_loop.block_on(lookup_future).unwrap();

        println!("Here3");
        let address: IpAddr = response.iter().next().expect("no addresses returned!");

println!("{address}")
}