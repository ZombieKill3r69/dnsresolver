use std::process::exit;

pub struct Config {
    pub query:      String,
    pub ip_ver:     u8,
}

pub fn parse_config(mut args: Vec<String>) -> Config {

    let mut flags: Vec<char> = Vec::new();

    // Remove flags from agruments
    if detect_flags(&args).is_flags {
        flags = (detect_flags(&args).result)
            .unwrap().chars()
            .skip(1).collect();
        args.remove((detect_flags(&args).position).unwrap());
    }

    if flags.contains(&'h') {
        display_help_info();
    }

    return Config {
        query:      args[0].clone(),
        ip_ver:     find_ip_version(&flags),
    };
}

struct FlagStatus {
    is_flags:   bool,
    position:   Option<usize>,
    result:     Option<String>,
}

// Returns the index of command line flags
fn detect_flags(args: &Vec<String>) -> FlagStatus {

    for (i, target_index) in args.iter().enumerate() {

        if target_index.starts_with('-') {

            return FlagStatus {
                is_flags: true, 
                position: Some(i), 
                result: Some(args[i].clone()),
            };
        }
    }

    return FlagStatus {
        is_flags: false, 
        position: None, 
        result: None,
    }
}

fn find_ip_version(flags: &Vec<char>) -> u8 {

    if flags.contains(&'6') {
        println!("Using ipv6");
        return 6;
    } else {
        println!("Using ipv4");
        return 4;
    }
}

fn display_help_info() {

    let mut output: String = String::new();

    output.push_str("usage: rresolve [options] query_address\n");
    output.push_str("  options:\n");
    output.push_str("    -4, Return Ipv4 (default)\n");
    output.push_str("    -6, Return Ipv6\n");
    output.push_str("    -h, Show this help text");

    print!("{}", output); 

    exit(0x0100)
}

