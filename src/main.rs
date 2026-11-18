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

    let output: String = resolver::resolve(config).to_string();

    // Output
    println!("returns: {}", output);
}

