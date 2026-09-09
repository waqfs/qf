mod model;

use crate::config::Config;
use std::path::Path;

pub fn build(root: &Path) -> Result<(), String> {
    let config = Config::load(root)?;
    Ok(())
}
