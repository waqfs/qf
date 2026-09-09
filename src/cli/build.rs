use std::path::PathBuf;

use crate::compiler;

pub fn run() -> Result<(), String> {
    let _ = compiler::build(PathBuf::from("test").as_path())?;
    Ok(())
}
