//use std::process;

pub struct Config {
    pub query:      String,
    //pub ip_ver:     u8,
}

pub fn parse_config(mut args: Vec<String>) -> Config {

    let flags: Vec<char> = (detect_flags(&args).0).chars().skip(1).collect();
    dbg!(flags);
    // Remove flags from agruments
    args.remove(detect_flags(&args).1);

    Config {
        query:      args[0].clone(),
        //ip_ver:     0,
    }
}

// Returns the index of command line flags
fn detect_flags(args: &Vec<String>) -> (String, usize) {

    for (i, target_index) in args.iter().enumerate() {

        dbg!(i);
        if target_index.starts_with('-') {

            return (args[i].clone(), i);
        }
    }
    (String::new(), args.len())
}
