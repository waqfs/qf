use std::env;

#[derive(Debug)]
enum Command {
    Build,
}

fn main() {
    println!("Hello, world!");
}

fn parse_command() -> Result<Command, String> {
    let mut arguments = env::args().skip(1);
    let command_raw = arguments
        .next()
        .ok_or_else(|| "Missing command argument.".to_string())?;
    match command_raw.as_str() {
        "build" => Ok(Command::Build),
        _ => Err(format!("Unknown command: {command_raw}")),
    }
}
