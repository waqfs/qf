mod cli;
mod compiler;
mod config;
mod parser;
mod staging;

use std::{env, eprintln, path::PathBuf, process::exit};

use crate::cli::CLIArguments;

#[derive(Debug)]
enum Command {
    Build(CLIArguments),
}

fn parse_command() -> Result<Command, String> {
    let mut arguments = env::args().skip(1);
    let command_raw = arguments
        .next()
        .ok_or_else(|| "Missing command argument.".to_string())?;

    let mut root = PathBuf::from(".");
    while let Some(arg) = arguments.next() {
        match arg.as_str() {
            "--root" => {
                let value = arguments
                    .next()
                    .ok_or_else(|| "--root requires a path".to_string())?;
                root = PathBuf::from(value);
            }
            unknown => return Err(format!("unknown argument: {unknown}")),
        }
    }

    let cli_arguments = CLIArguments { root };
    match command_raw.as_str() {
        "build" => Ok(Command::Build(cli_arguments)),
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
        Command::Build(arguments) => cli::build::run(arguments),
    } {
        eprintln!("Error: {error}");
        exit(1);
    }
}
