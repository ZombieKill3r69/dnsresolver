use std::process;

pub struct Config {
    pub query:      String,
}

pub fn parse_config(mut args: Vec<String>) -> Config {

    // Remove flags from arfuments
    args.remove(detect_flags(&args).1);

    Config {
        query:      args[0].clone()
    }
}

// Returns the index of command line flags
fn detect_flags(args: &Vec<String>) -> (String, usize) {

    for (i, target_index) in args.iter().enumerate() {

        if target_index.starts_with('-') {

            return (args[i].clone(), i);
            
        }

    }

    (String::new(), args.len())
}