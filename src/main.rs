mod commands;
mod parser;

fn main() -> Result<(), String> {
    let inputs: Vec<String> = std::env::args().skip(1).collect();
    if inputs.is_empty() {
        return Err("No input given.".parse().unwrap());
    }
    let input_array = &inputs[..];
    if let Some(cmd) = parser::command(input_array) {
        let args = parser::args(input_array);
        let opt = parser::options(input_array);
        match cmd {
            parser::Commands::Help => commands::help(),
            parser::Commands::AddEnvPath => commands::add_env_path(args, opt)?,
            parser::Commands::AddEnvVar => commands::add_env_var(args, opt)?,
        }
    } else {
        return Err("Command not found.".parse().unwrap());
    }
    Ok(())
}
