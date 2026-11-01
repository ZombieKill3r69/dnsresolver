use std::env;

mod cli;
mod resolver;

fn main() {

    let args: Vec<String> = env::args().collect();
    let user_args: &[String] = &args[1..];

    let config: cli::Config = cli::parse_config(user_args);

    let output: String = resolver::resolve(&config.query).to_string();

    println!("{} returns: {}", config.query, output);
}
