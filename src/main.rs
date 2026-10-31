use std::env;

mod cli;
mod resolver;

fn main() {

    let search: &str = "example.com";
    let args: Vec<String> = env::args().collect();

    let config: cli::Config= cli::parse_config(&args);

    let output: String = resolver::resolve(&config.query).to_string();

    println!("{search} returns: {output}");
}
