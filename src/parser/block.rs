use std::path::Path;

use crate::compiler::model::site::Block;

pub fn parse(path: &Path, lines: &[&str]) -> Result<Vec<Block>, String> {
    let mut blocks: Vec<Block> = Vec::new();
    let mut index = 0;

    std::process::exit(1);
}
