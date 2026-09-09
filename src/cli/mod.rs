use std::path::PathBuf;

pub mod build;

#[derive(Debug, Clone)]
pub struct CLIArguments {
    pub root: PathBuf,
}
