mod resolver;

fn main() {

    let search: &str = "ksdjhfkjshfsjdfh";

    let output: String = resolver::resolve(search).to_string();

    println!("{search} returns: {output}")
}
