use std::{collections::BTreeSet, path::Path};

use crate::compiler::model::site::{DocumentType, Metadata};

pub fn parse(path: &Path, lines: &[&str]) -> Result<(Metadata, usize), String> {
    let mut seen: BTreeSet<String> = BTreeSet::new();
    let mut doc_type: Option<DocumentType> = None;
    let mut title: Option<String> = None;
    let mut summary: Option<String> = None;
    let mut date: Option<String> = None;
    let mut index: usize = 0;

    std::process::exit(1);
}
