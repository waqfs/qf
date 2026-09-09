mod cli;
mod compiler;
mod config;

use std::{env, eprintln, process::exit};

#[derive(Debug)]
enum Command {
    Build,
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

fn main() {
    let command = match parse_command() {
        Ok(command) => command,
        Err(error) => {
            eprintln!("Error: {error}");
            exit(1);
        }
    };
    if let Err(error) = match command {
        Command::Build => cli::build::run(),
    } {
        eprintln!("Error: {error}");
        exit(1);
    }
}
