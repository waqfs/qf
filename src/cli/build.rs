use std::fs;

use crate::{cli::CLIArguments, compiler, error::QFError};

const DEFAULT_CONFIG: &str = include_str!("default/qf.config.json");
const DEFAULT_STYLESHEET: &str = include_str!("default/style.css");
const DEFAULT_INDEX: &str = include_str!("default/index.qf");

pub fn run(arguments: CLIArguments) -> Result<(), QFError> {
    compiler::build(&arguments)?;
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

    let content_path = arguments.root.join("content");
    if !content_path.exists() {
        fs::create_dir_all(&content_path)?;
    }

    let index_qf_path = content_path.join("index.qf");
    if !index_qf_path.exists() {
        fs::write(index_qf_path, DEFAULT_INDEX)?;
    }
    Ok(())
}
