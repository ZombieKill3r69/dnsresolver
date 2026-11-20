use std::env;

mod cli;
mod resolver;

pub struct Config {
    pub query:      String,
    pub ip_ver:     u8,
}

fn main() {

    // Get command line arguments
    let mut args: Vec<String> = env::args().collect();

    // Removed the executable name from arguments
    args.remove(0);

    let config: cli::Config = cli::parse_config(args);

    let output: String = resolver::resolve(&config);

    // Output
    println!("{}", format_output(config.query, output));
}

fn format_output(server_address: String, server_output: String) -> String {

    if server_output.contains("Unable") {
        return server_output;
    } else {
        return format!("Address for {server_address} is {server_output}")
    }
}
