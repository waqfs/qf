use std::collections::BTreeSet;

use chrono::{DateTime, Utc};

use crate::{
    compiler::model::site::{DocumentType, Metadata},
    config::Config,
    error::QFError,
    parser::date::parse_date,
};

pub fn parse(lines: &[&str], config: &Config) -> Result<(Metadata, usize), QFError> {
    let mut seen: BTreeSet<String> = BTreeSet::new();
    let mut doc_type: Option<DocumentType> = None;
    let mut title: Option<String> = None;
    let mut summary: Option<String> = None;
    let mut date: Option<DateTime<Utc>> = None;
    let mut author: Option<String> = None;
    let mut handle: Option<String> = None;
    let mut starred: bool = false;
    let mut is_index: bool = false;
    let mut is_draft: bool = false;
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

        let (key, value) = line[1..].split_once(' ').ok_or_else(|| {
            QFError::Message(format!(
                "line {}: Failed to parse metadata property",
                index + 1
            ))
        })?;

        if !seen.insert(key.to_string()) {
            return Err(QFError::Message(format!(
                "line {}: Duplicate metadata property {key}",
                index + 1
            )));
        }

        match key {
            "type" => {
                doc_type = Some(match value.trim() {
                    "page" => DocumentType::Page,
                    "project" => DocumentType::Project,
                    "article" => DocumentType::Article,
                    unknown => {
                        return Err(QFError::Message(format!(
                            "line {}: Unknown document type {unknown}",
                            index + 1
                        )));
                    }
                })
            }
            "title" => title = Some(value.trim().to_string()),
            "summary" => summary = Some(value.trim().to_string()),
            "date" => date = Some(parse_date(value, &config.timezone)?),
            "author" => author = Some(value.trim().to_string()),
            "handle" => handle = Some(value.trim().to_string()),
            "starred" => {
                starred = match value.trim() {
                    "true" => true,
                    "false" => false,
                    unknown => {
                        return Err(QFError::Message(format!(
                            "line {}: Unknown starred type {unknown}",
                            index + 1
                        )));
                    }
                }
            }
            "index" => {
                is_index = match value.trim() {
                    "true" => true,
                    "false" => false,
                    unknown => {
                        return Err(QFError::Message(format!(
                            "line {}: Unknown index type {unknown}",
                            index + 1
                        )));
                    }
                }
            }
            "draft" => {
                is_draft = match value.trim() {
                    "true" => true,
                    "false" => false,
                    unknown => {
                        return Err(QFError::Message(format!(
                            "line {}: Unknown draft type {unknown}",
                            index + 1
                        )));
                    }
                }
            }
            unknown => {
                return Err(QFError::Message(format!(
                    "line {}: Unknown metadata property {unknown}",
                    index + 1
                )));
            }
        }

        index += 1;
    }

    Ok((
        Metadata {
            doc_type: doc_type.ok_or_else(|| {
                QFError::Message("Missing required 'type' metadata property".into())
            })?,
            title: title.ok_or_else(|| {
                QFError::Message("Missing required 'title' metadata property".into())
            })?,
            summary,
            date,
            author,
            handle,
            starred,
            is_index,
            is_draft,
        },
        index,
    ))
}
