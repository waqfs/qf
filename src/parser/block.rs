use std::path::Path;

use crate::{
    compiler::model::site::{Block, Image, ImageAlt},
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
