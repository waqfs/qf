mod block;
mod inline;
mod metadata;

use crate::compiler::{model::site::Document, source::PortfolioFile};

pub fn parse_document(file: &PortfolioFile) -> Result<Document, String> {
    let lines: Vec<&str> = file.raw.lines().collect();
    let (metadata, next_pos) = metadata::parse(&file.path, &lines)?;
    let blocks = block::parse(&file.path, &lines[next_pos..])?;
    Ok(Document {
        source_path: file.path.clone(),
        metadata: metadata,
        route: String::new(),
        blocks: blocks,
    })
}
