use std::{fs, path::Path};

use crate::{
    compiler::model::{
        document::Site,
        site::{
            Block::{self},
            ImageAlt, Inline,
        },
    },
    config::Config,
    output::OutputFormatter,
};

pub struct TextOutputFormatter;

impl OutputFormatter for TextOutputFormatter {
    fn name(&self) -> &'static str {
        "text"
    }

    fn write(&self, site: &Site, config: &Config, stage: &Path) -> Result<(), String> {
        for document in &site.documents {
            let dir = stage.join(document.route.trim_matches('/'));
            fs::create_dir_all(&dir)
                .map_err(|e| format!("Failed to create output directory: {e}"))?;

            let mut text = String::new();
            text.push_str(&document.metadata.title.to_uppercase());
            text.push('\n');
            text.push_str(&"=".repeat(document.metadata.title.chars().count()));
            text.push('\n');
            if let Some(summary) = &document.metadata.summary {
                text.push_str(summary);
                text.push_str("\n\n");
            } else {
                text.push('\n');
            }
            for block in &document.blocks {
                write_block(block, &mut text);
            }

            if let Some(route) = &document.previous {
                text.push_str(&format!("Previous: {}", route));
            }
            if let Some(route) = &document.next {
                text.push_str(&format!("Next: {}", route));
            }

            fs::write(dir.join("index.txt"), text)
                .map_err(|e| format!("Failed to write file: {e}"))?;
        }
        Ok(())
    }
}

fn write_block(block: &Block, output: &mut String) {
    match block {
        Block::Heading { level, content } => {
            let mut heading = String::new();
            write_inlines(content, &mut heading);
            output.push_str(&heading);
            output.push('\n');
            output.push_str(&if *level == 1 { "-" } else { "~" }.repeat(heading.chars().count()));
            output.push_str("\n");
        }
        Block::UnorderedList(items) => {
            for item in items {
                output.push_str("- ");
                write_inlines(item, output);
                output.push('\n');
            }
            output.push('\n');
        }
        Block::LinkedIndex(crate::compiler::model::site::LinkedIndex { items, .. })
        | Block::OrderedList(items) => {
            for (index, item) in items.iter().enumerate() {
                output.push_str(&format!("{}. ", index + 1));
                write_inlines(item, output);
                output.push('\n');
            }
            output.push('\n');
        }
        Block::BlockQuote(inlines) => {
            output.push_str("> ");
            write_inlines(inlines, output);
            output.push_str("\n\n");
        }
        Block::CodeBlock { language, code } => {
            if let Some(language) = &language {
                output.push_str("[");
                output.push_str(language);
                output.push_str("]\n");
            }
            let width = code.lines().count().to_string().len();
            for (index, line) in code.lines().enumerate() {
                output.push_str(&format!("{:>width$} | ", index + 1));
                output.push_str(line);
                output.push('\n');
            }
            output.push('\n');
        }
        Block::Image(image) => {
            if let ImageAlt::Description(alt) = &image.alt {
                output.push_str("[Image: ");
                output.push_str(alt);
                output.push_str("] @ ");
                output.push_str(&image.source);
                output.push('\n');
            }
            if let Some(caption) = &image.caption {
                output.push_str(caption);
                output.push('\n');
            }
            output.push('\n');
        }
        Block::Paragraph(inlines) => {
            write_inlines(inlines, output);
            output.push_str("\n\n");
        }
    }
}

fn write_inlines(inlines: &[Inline], output: &mut String) {
    for inline in inlines {
        match inline {
            Inline::Text(text) | Inline::Code(text) => output.push_str(text),
            Inline::Emphasis(text) | Inline::Strong(text) => write_inlines(text, output),
            Inline::Link { label, href } => {
                write_inlines(label, output);
                output.push_str(" (");
                output.push_str(href);
                output.push(')');
            }
        }
    }
}
