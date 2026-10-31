mod resolver;

fn main() {

    let search: &str = "example.com";

    let output: String = resolver::resolve(search).to_string();

    println!("{search} returns: {output}")
}
