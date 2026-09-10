use std::{collections::BTreeSet, path::Path};

use crate::compiler::model::site::{DocumentType, Metadata};

pub fn parse(path: &Path, lines: &[&str]) -> Result<(Metadata, usize), String> {
    let mut seen: BTreeSet<String> = BTreeSet::new();
    let mut doc_type: Option<DocumentType> = None;
    let mut title: Option<String> = None;
    let mut summary: Option<String> = None;
    let mut date: Option<String> = None;
    let mut index: usize = 0;

    while index < lines.len() {
        let line = lines[index].trim();
        if line.is_empty() {
            index += 1;
            continue;
        }
        if !line.starts_with('@') {
            break;
        }

        let (key, value) = line[1..]
            .split_once(' ')
            .ok_or_else(|| format!("Failed to parse metadata property"))?;

        if !seen.insert(key.to_string()) {
            return Err(format!(
                "Duplicate metadata property {key} in {}",
                path.display()
            ));
        }

        match key {
            "type" => {
                doc_type = Some(match value.trim() {
                    "page" => DocumentType::Page,
                    "project" => DocumentType::Project,
                    "article" => DocumentType::Article,
                    unknown => {
                        return Err(format!(
                            "Unknown document type {unknown} in {}",
                            path.display()
                        ));
                    }
                })
            }
            "title" => title = Some(value.trim().to_string()),
            "summary" => summary = Some(value.trim().to_string()),
            "date" => date = Some(value.trim().to_string()),
            unknown => {
                return Err(format!(
                    "Unknown metadata property {unknown} in {}",
                    path.display()
                ));
            }
        }

        index += 1;
    }

    Ok((
        Metadata {
            doc_type: doc_type.ok_or_else(|| {
                format!(
                    "Missing required 'type' metadata property in {}",
                    path.display()
                )
            })?,
            title: title.ok_or_else(|| {
                format!(
                    "Missing required 'title' metadata property in {}",
                    path.display()
                )
            })?,
            summary,
            date,
        },
        index,
    ))
}
