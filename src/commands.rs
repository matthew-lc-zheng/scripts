#[cfg(target_os = "windows")]
use powershell_script;

// #[cfg(target_family = "unix")]

pub fn help() {
    println!("USAGE: 'st --command -option arg1 arg2'.");
    println!("Only one command can be supplied.");
    println!("options and arguments are free to add.");
    println!("Invalid options will be ignored automatically.");
    println!("Arguments should be correctly given by usage of certain commands.");
}

fn usage_add_env_path() {
    println!("USAGE: -h: get usages.");
    println!("Default add given path to user's path.");
    println!("-sys: add given path to system's path, which requires root privileges.");
    println!("Example: 'st --add_env_path <path>' or 'st --add_env_path <path> -sys'.");
}

fn check_sys_option(opts: &Option<Vec<String>>) -> bool {
    if let Some(_opt) = opts {
        for opt in _opt {
            if opt == "-sys" {
                return true;
            }
        }
    }
    false
}

#[cfg(target_family = "unix")]
fn check_config_path(config_path: &mut String, sys_mode: bool) -> Result<(), std::io::Error> {
    *config_path = if sys_mode {
        String::from("/etc/profile")
    } else {
        std::env::var("HOME").unwrap() + "/.bashrc"
    };
    if let Err(e) = std::fs::metadata(&config_path) {
        if e.kind() != std::io::ErrorKind::NotFound {
            return Err(e);
        }
        if let Err(e) = std::fs::File::create(&config_path) {
            return Err(e);
        }
    }
    Ok(())
}

pub fn add_env_path(args: Option<Vec<String>>, opts: Option<Vec<String>>) -> Result<(), String> {
    let Some(_args) = args else {
        return Err("no arguments given".parse().unwrap());
    };
    let sys_mode = check_sys_option(&opts);

    #[cfg(target_os = "windows")]
    let ps = powershell_script::PsScriptBuilder::new()
        .no_profile(true)
        .non_interactive(true)
        .hidden(true)
        .print_commands(false)
        .build();
    #[cfg(target_os = "windows")]
    let mode = if sys_mode { "Machine" } else { "User" };

    #[cfg(target_family = "unix")]
    let mut config_path: String = String::new();

    // #[cfg(target_family = "unix")]
    // if let Err(e) = check_config_path(&mut config_path, sys_mode) {
    //     return Err(format!(
    //         "invalid config path {}, error: {}",
    //         &config_path, &e
    //     ));
    // }

    let mut path = String::new();
    for arg in &_args {
        #[cfg(target_family = "unix")]
        {
            path += ":";
        }

        path += arg;

        #[cfg(target_os = "windows")]
        {
            path += ";";
        }
    }

    println!("@@Path: {}", path);

    #[cfg(target_os = "windows")]
    {
        let Ok(env_path) = std::env::var("Path") else {
            return Err("Path not set".parse().unwrap());
        };
        path = env_path + &path;
    }

    #[cfg(target_os = "windows")]
    if let Err(_) = ps.run(&format!(
        r#"[Environment]::SetEnvironmentVariable("Path", {}, [EnvironmentVariableTarget]::{})"#,
        path, mode
    )) {
        return Err("Failed to set environment path".parse().unwrap());
    }

    #[cfg(target_family = "unix")]
    {
        let output = std::process::Command::new("echo")
            .arg(format!("\"export PATH=$PATH{}\"", path))
            .arg(">>")
            .arg(&config_path)
            .arg("&&")
            .arg("source")
            .arg(&config_path)
            .output()
            .expect("Failed to execute shell command");
        if !output.status.success() {
            return Err(format!(
                "Failed to execute shell command, error:{}",
                String::from_utf8_lossy(&output.stderr)
            ));
        }
        println!("{}", String::from_utf8_lossy(&output.stdout));
    }

    Ok(())
}

fn usage_add_env_var() {
    println!("USAGE: st add-env [PATH]");
}

pub fn add_env_var(args: Option<Vec<String>>, opts: Option<Vec<String>>) -> Result<(), String> {
    let sys_mode = check_sys_option(&opts);

    let Some(_args) = args else {
        return Err("no arguments given".parse().unwrap());
    };
    #[cfg(target_os = "windows")]
    let ps = powershell_script::PsScriptBuilder::new()
        .no_profile(true)
        .non_interactive(true)
        .hidden(true)
        .print_commands(false)
        .build();
    #[cfg(target_os = "windows")]
    let mode = if sys_mode { "Machine" } else { "User" };

    #[cfg(target_family = "unix")]
    let mut config_path = String::new();

    #[cfg(target_family = "unix")]
    if let Err(e) = check_config_path(&mut config_path, sys_mode) {
        return Err(format!(
            "invalid config path {}, error: {}",
            &config_path, &e
        ));
    }

    for arg in &_args {
        let parts: Vec<&str> = arg.split(':').collect();
        if parts.len() != 2 {
            eprintln!("{} -> failed: not a valid format.", arg);
            continue;
        }
        let var = parts[0].trim();
        let val = parts[1].trim();

        #[cfg(target_os = "windows")]
        if let Ok(_) = ps.run(&format!(
            r#"[Environment]::SetEnvironmentVariable({},{}, [EnvironmentVariableTarget]::{})"#,
            var, val, mode
        )) {
            println!("{} -> successful.", arg);
        } else {
            eprintln!("{} -> failed.", arg);
        }

        #[cfg(target_family = "unix")]
        {
            let output = std::process::Command::new("echo")
                .arg(format!("\"export {}=\"{}\"", var, val))
                .arg(">>")
                .arg(&config_path)
                .output()
                .expect("Failed to execute shell command");
            if !output.status.success() {
                return Err(format!(
                    "Failed to execute shell command, error:{}",
                    String::from_utf8_lossy(&output.stderr)
                ));
            }
        }
    }

    #[cfg(target_family = "unix")]
    {
        let output = std::process::Command::new("source")
            .arg(&config_path)
            .output()
            .expect("Failed to execute shell command");
        if !output.status.success() {
            return Err(format!(
                "Failed to execute shell command, error:{}",
                String::from_utf8_lossy(&output.stderr)
            ));
        }
    }

    Ok(())
}
