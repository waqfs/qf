use std::{fs, path::Path};

use crate::{
    compiler::model::{
        document::Site,
        site::{Block, Document},
    },
    config::Config,
    output::OutputFormatter,
};

pub struct HTMLOutputFormatter;

impl OutputFormatter for HTMLOutputFormatter {
    fn name(&self) -> &'static str {
        "html"
    }

    fn write(&self, site: &Site, config: &Config, stage: &Path) -> Result<(), String> {
        for document in &site.documents {
            let dir = stage.join(document.route.trim_matches('/'));
            fs::create_dir_all(&dir)
                .map_err(|e| format!("Failed to create output directory: {e}"))?;
            let text = write_document(document, site, config);
            fs::write(dir.join("index.html"), text)
                .map_err(|e| format!("Failed to write file: {e}"))?;
        }
        Ok(())
    }
}

fn write_document(document: &Document, site: &Site, config: &Config) -> String {
    let mut body = String::new();

    write_head(document, config, &mut body);
    body.push_str("<body>\n");

    body.push_str("<div id=\"content\">");
    body.push_str("<main>\n");
    body.push_str("<article>\n");
    body.push_str("<header>\n");
    body.push_str(&format!(
        "<h1>{}</h1>",
        html_escape(&document.metadata.title)
    ));
    if let Some(summary) = &document.metadata.summary {
        body.push_str(&format!("<p>{}</p>\n", html_escape(summary)));
    }
    if let Some(date) = &document.metadata.date {
        body.push_str(&format!(
            "<time datetime=\"{}\">{}</time>\n",
            html_escape_attribute(date),
            html_escape(date)
        ));
    }
    body.push_str("</header>\n");
    for block in &document.blocks {
        write_block(block, &mut body);
    }
    body.push_str("</article>\n");
    body.push_str("</main>\n");
    body.push_str("</div>\n");

    body.push_str("</body>\n");
    body.push_str("</html>\n");
    body
}

fn write_head(document: &Document, config: &Config, output: &mut String) {
    output.push_str("<!DOCTYPE html>");
    output.push_str(&format!("<html lang=\"{}\">\n", config.language.as_str()));
    output.push_str("<head>\n");
    output.push_str("<meta charset=\"utf-8\">\n");
    output.push_str("<meta name=\"viewport\" content=\"width=device-width, initial-scale=1\">\n");
    output.push_str("<meta name=\"color-scheme\" content=\"light dark\">\n");
    output.push_str(&format!("<title>{}</title>\n", "placeholder"));
    output.push_str(&format!(
        "<meta name=\"description\" content=\"{}\">\n",
        "plaecholder"
    ));
    output.push_str(&format!(
        "<link rel=\"stylesheet\" href=\"{}\">\n",
        "placeholder"
    ));
    output.push_str("</head>\n");
}

fn write_block(block: &Block, output: &mut String) {}

fn html_escape(value: &str) -> String {
    value
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#39;")
}

fn html_escape_attribute(value: &str) -> String {
    value.replace('&', "&amp;").replace('"', "&quot;")
}
