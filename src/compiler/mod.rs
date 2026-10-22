pub mod index;
pub mod model;
pub mod source;

use crate::{
    compiler::{
        index::{build_index, process_index},
        model::document::Site,
    },
    config::Config,
    error::QFError,
    output::outputs,
    parser, staging,
};
use std::path::Path;

pub fn build(root: &Path) -> Result<(), QFError> {
    let (config, site) = compile(root)?;
    let stage = staging::init_stage(root, &config)?;

    for output in outputs() {
        output.write(&site, &config, &stage)?;
    }

    staging::insert_static(root, &config, &stage)?;
    staging::commit_stage(root, &config, &stage)?;
    Ok(())
}

fn compile(root: &Path) -> Result<(Config, Site), QFError> {
    let config = Config::load(root)?;
    let files = source::get_portfolio_files(root, &config)?;
    let mut documents = Vec::new();

    for file in files {
        match source::get_raw(&file).and_then(|file| parser::parse_document(&file, &config)) {
            Ok(document) => {
                documents.push(document);
            }
            Err(error) => {
                return Err(QFError::Message(format!("{}: {error}", file.display())));
            }
        }
    }

    let mut site: Site = build_index(root, config.clone(), documents)?;
    process_index(&mut site)?;
    Ok((config, site))
}
