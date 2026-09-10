pub mod model;
pub mod source;

use crate::{compiler::model::document::Site, config::Config, output::outputs, parser, staging};
use std::path::Path;

pub fn build(root: &Path) -> Result<(), String> {
    let (config, site) = compile(root)?;
    let stage = staging::init_stage(root, &config)?;

    for output in outputs() {
        output.write(&site, &config, &stage)?;
    }

    staging::commit_stage(root, &config, &stage)?;
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
