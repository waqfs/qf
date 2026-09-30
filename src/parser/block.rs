use std::{collections::BTreeSet, path::Path};

use crate::{
    compiler::model::site::{Block, DocumentType, Image, ImageAlt, LinkedIndex, MetadataFields},
    parser::inline,
};

pub fn parse(path: &Path, lines: &[&str]) -> Result<Vec<Block>, String> {
    let mut blocks: Vec<Block> = Vec::new();
    let mut index = 0;

    while index < lines.len() {
        let line = lines[index];
        let trimmed = line.trim();

        if trimmed.is_empty() {
            index += 1;
            continue;
        }

        // Linked Indexes
        if line_is_linked_index(trimmed).is_some() {
            blocks.push(Block::LinkedIndex(parse_linked_index(trimmed)?));
            index += 1;
            continue;
        }

        // Headers
        if trimmed.starts_with('#') {
            let count = trimmed.chars().take_while(|chr| *chr == '#').count();
            if count > 6 || trimmed.chars().nth(count) != Some(' ') {
                return Err(format!(
                    "Maximum of 6 hashes and a space required for headers in {}",
                    path.display()
                ));
            }
            blocks.push(Block::Heading {
                level: count as u8,
                content: inline::parse(trimmed[count + 1..].trim()),
            });
            index += 1;
            continue;
        }

        // Unordered List
        if trimmed.starts_with("- ") {
            let mut items = Vec::new();
            while index < lines.len() {
                let item = lines[index].trim();
                let Some(value) = item.strip_prefix("- ") else {
                    break;
                };
                items.push(inline::parse(value));
                index += 1;
            }
            blocks.push(Block::UnorderedList(items));
            continue;
        }

        // Ordered List
        if line_is_ordered_list(trimmed) {
            let mut items = Vec::new();
            while index < lines.len() && line_is_ordered_list(lines[index].trim()) {
                let item = lines[index].trim();
                let value = item.split_once(". ").unwrap().1;
                items.push(inline::parse(value));
                index += 1;
            }
            blocks.push(Block::OrderedList(items));
            continue;
        }

        // Quotes
        if let Some(value) = trimmed.strip_prefix("> ") {
            blocks.push(Block::BlockQuote(inline::parse(value)));
            index += 1;
            continue;
        }

        // Code Blocks
        if let Some(language) = trimmed.strip_prefix("```") {
            let language = if language.trim().is_empty() {
                None
            } else {
                Some(language.trim().to_string())
            };
            index += 1;
            let mut code = String::new();
            while index < lines.len() && lines[index].trim() != "```" {
                if !code.is_empty() {
                    code.push('\n');
                }
                code.push_str(lines[index]);
                index += 1;
            }
            if index == lines.len() {
                return Err(format!("Code block not closed in {}", path.display()));
            }
            index += 1;
            blocks.push(Block::CodeBlock { language, code });
            continue;
        }

        // Images
        if let Some(href) = trimmed.strip_prefix("@image ") {
            let source = href.trim().to_string();
            index += 1;
            let mut alt: Option<ImageAlt> = None;
            let mut caption: Option<String> = None;

            while index < lines.len() && !lines[index].trim().is_empty() {
                let property = lines[index].trim();
                if property == "decorative" {
                    alt = Some(ImageAlt::Decorative);
                } else if let Some(value) = parse_image_property(property, "alt") {
                    alt = Some(ImageAlt::Description(value));
                } else if let Some(value) = parse_image_property(property, "caption") {
                    caption = Some(value);
                } else {
                    return Err(format!(
                        "Unknown image property {property} in {}",
                        path.display()
                    ));
                }
                index += 1;
            }

            let alt =
                alt.ok_or_else(|| format!("Image requires an alt property in {}", path.display()))?;
            blocks.push(Block::Image(Image {
                source,
                alt,
                caption,
            }));
            continue;
        }

        // Paragraph
        let mut paragraph = String::new();
        while index < lines.len() {
            let line = lines[index].trim();
            if line.is_empty() || line_starts_block(line) {
                break;
            }
            if !paragraph.is_empty() {
                paragraph.push(' ');
            }
            paragraph.push_str(line);
            index += 1;
        }
        blocks.push(Block::Paragraph(inline::parse(&paragraph)));
    }

    Ok(blocks)
}

fn line_starts_block(line: &str) -> bool {
    line.starts_with('#')
        || line.starts_with("- ")
        || line.starts_with("> ")
        || line.starts_with("@image ")
        || line.starts_with("```")
        || line_is_ordered_list(line)
}

fn line_is_ordered_list(line: &str) -> bool {
    let Some((left, _)) = line.split_once(". ") else {
        return false;
    };
    !left.is_empty() && left.chars().all(|chr| chr.is_ascii_digit())
}

fn parse_image_property(line: &str, key: &str) -> Option<String> {
    let prefix = format!("{key}=\"");
    if line.starts_with(&prefix) && line.ends_with('"') {
        Some(line[prefix.len()..line.len() - 1].to_string())
    } else {
        None
    }
}

fn line_is_linked_index(line: &str) -> Option<DocumentType> {
    match line.split_whitespace().next()? {
        "@projects" => Some(DocumentType::Project),
        "@articles" => Some(DocumentType::Article),
        _ => None,
    }
}

fn parse_linked_index(text: &str) -> Result<LinkedIndex, String> {
    let mut index = LinkedIndex::new(line_is_linked_index(text).unwrap());
    let mut visited: BTreeSet<&str> = BTreeSet::new();

    for option in text.split_whitespace().skip(1) {
        let (key, value) = option
            .split_once('=')
            .ok_or_else(|| format!("expected key=value for {option}"))?;

        if !visited.insert(key) {
            return Err(format!("duplicate key {key}"));
        }

        match key {
            "limit" => {
                if value.is_empty() || !value.chars().all(|chr| chr.is_ascii_digit()) {
                    return Err(format!("key 'limit' requires positive integer"));
                }
                index.limit = Some(
                    value
                        .parse()
                        .map_err(|err| format!("key 'limit' failed to parse integer: {err}"))?,
                );
            }
            "starred" => {
                index.starred = Some(match value {
                    "true" => true,
                    "false" => false,
                    unknown => return Err(format!("key 'starred' has unknown value {unknown}")),
                })
            }
            "fields" => {
                index.fields.clear();
                for field in value.split(',') {
                    let field = match field {
                        "title" => MetadataFields::Title,
                        "summary" => MetadataFields::Summary,
                        "date" => MetadataFields::Date,
                        unknown => return Err(format!("key 'fields' has unknown value {unknown}")),
                    };
                    if index.fields.contains(&field) {
                        return Err(format!("key 'fields' has duplicate value"));
                    }
                    index.fields.push(field);
                }
            }
            unknown => return Err(format!("unknown option key '{unknown}'")),
        }
    }

    Ok(index)
}
