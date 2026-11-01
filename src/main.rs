use std::env;

mod cli;
mod resolver;

fn main() {

    // Get command line arguments
    let mut args: Vec<String> = env::args().collect();
    // Removed the executable name from arguments
    args.remove(0);

    let config: cli::Config = cli::parse_config(args);

    let output: String = resolver::resolve(&config.query).to_string();

    // Output
    println!("{} returns: {}", config.query, output);
}
