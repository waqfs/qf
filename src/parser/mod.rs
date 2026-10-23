pub mod block;
pub mod date;
mod inline;
mod metadata;

use crate::{
    compiler::{model::site::Document, source::PortfolioFile},
    config::Config,
    error::QFError,
};

pub fn parse_document(file: &PortfolioFile, config: &Config) -> Result<Document, QFError> {
    let lines: Vec<&str> = file.raw.lines().collect();
    let (metadata, next_pos) = metadata::parse(&lines, config)?;
    let blocks = block::parse(&lines[next_pos..])?;
    Ok(Document {
        source_path: file.path.clone(),
        metadata,
        route: String::new(),
        blocks,
        next: None,
        all: None,
        previous: None,
    })
}
