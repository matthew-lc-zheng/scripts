pub enum Commands {
    Help,
    AddEnvPath,
    AddEnvVar,
}

pub fn command(inputs: &[String]) -> Option<Commands> {
    for input in inputs {
        if !input.starts_with("--") {
            continue;
        }
        match input.as_str() {
            "--add_env_path" => return Some(Commands::AddEnvPath),
            "--add_env_var" => return Some(Commands::AddEnvVar),
            "--help" => return Some(Commands::Help),
            _ => continue,
        }
    }
    None
}

pub fn options(inputs: &[String]) -> Option<Vec<String>> {
    let mut opt = vec![];
    for input in inputs {
        if input.starts_with("-") {
            opt.push(String::from(input));
        }
    }
    match opt.len() {
        0 => None,
        _ => Some(opt),
    }
}

pub fn args(inputs: &[String]) -> Option<Vec<String>> {
    let mut args = vec![];
    for input in inputs {
        if !input.starts_with("-") && !input.starts_with("-") {
            args.push(String::from(input));
        }
    }
    match args.len() {
        0 => None,
        _ => Some(args),
    }
}
