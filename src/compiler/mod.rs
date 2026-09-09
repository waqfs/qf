mod model;
mod source;

use crate::{compiler::model::document::Site, config::Config};
use std::path::Path;

pub fn build(root: &Path) -> Result<(), String> {
    compile(root);
    Ok(())
}

fn compile(root: &Path) -> Result<(Config, Site), String> {
    let config = Config::load(root)?;
    let files = source::get_portfolio_files(root, &config)?;
    std::process::exit(1);
}
