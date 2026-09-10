pub mod model;
pub mod source;

use crate::{compiler::model::document::Site, config::Config, parser};
use std::path::Path;

pub fn build(root: &Path) -> Result<(), String> {
    compile(root);
    Ok(())
}

fn compile(root: &Path) -> Result<(Config, Site), String> {
    let config = Config::load(root)?;
    let files = source::get_portfolio_files(root, &config)?;
    let mut documents = Vec::new();

    for file in files {
        match source::get_raw(&file).and_then(|file| parser::parse_document(&file)) {
            Ok(document) => {
                documents.push(document);
            }
            Err(error) => return Err(format!("Error parsing file {}: {}", file.display(), error)),
        }
    }

    std::process::exit(1);
}
