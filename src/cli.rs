//use std::process;

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
    
    
    Config {
        query:      args[0].clone(),
        ip_ver:     get_ip_ver(&flags),
    }
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

    // Return empty values if nothing is found
    return FlagStatus {
        is_flags: false,
        position: None,
        result: None,
    }
}

fn get_ip_ver(user_flags: &Vec<char>) -> u8 {

    if user_flags.contains(&'6') {
        return 6;
    } else {
        return 4;
    }
}
