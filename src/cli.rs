pub struct Config {
    pub query:  String,
}

pub fn parse_config(args: &[String]) -> Config {
    
    Config {
        query: args[1].clone()
    }
}