use std::fs;

use crate::{cli::CLIArguments, compiler, error::QFError};

const DEFAULT_CONFIG: &str = include_str!("default/qf.config.json");
const DEFAULT_STYLESHEET: &str = include_str!("default/style.css");

pub fn run(arguments: CLIArguments) -> Result<(), QFError> {
    let _ = compiler::build(&arguments.root)?;
    Ok(())
}

pub fn init(arguments: CLIArguments) -> Result<(), QFError> {
    if !arguments.root.exists() {
        fs::create_dir_all(&arguments.root)?;
    }

    let config_path = arguments.root.join("qf.config.json");
    if !config_path.exists() {
        fs::write(config_path, DEFAULT_CONFIG)?;
    }

    let static_path = arguments.root.join("static");
    if !static_path.exists() {
        fs::create_dir_all(&static_path)?;
    }

    let stylesheet_path = static_path.join("style.css");
    if !stylesheet_path.exists() {
        fs::write(stylesheet_path, DEFAULT_STYLESHEET)?;
    }
    Ok(())
}
