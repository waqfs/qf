use std::{fs, path::Path};

use crate::{
    compiler::model::{document::Site, site::Document},
    config::Config,
    output::OutputFormatter,
};

pub struct RSSOutputFormatter;

impl OutputFormatter for RSSOutputFormatter {
    fn name(&self) -> &'static str {
        "rss"
    }

    fn write(&self, site: &Site, config: &Config, stage: &Path) -> Result<(), String> {
        let mut documents: Vec<&Document> = Vec::new();
        if let Some(index) = site.article_index {
            documents.push(&site.documents[index]);
        }
        if let Some(index) = site.project_index {
            documents.push(&site.documents[index]);
        }

        for document in documents {
            let dir = stage.join(document.route.trim_matches('/'));
            fs::create_dir_all(&dir)
                .map_err(|e| format!("Failed to create output directory: {e}"))?;
            let text = write_document(document, site, config);
            fs::write(dir.join("rss.xml"), text)
                .map_err(|e| format!("Failed to write file: {e}"))?;
        }

        Ok(())
    }
}

fn write_document(document: &Document, site: &Site, config: &Config) -> String {
    let mut body = String::new();
    body.push_str("rss");
    body
}

fn xml_escape(value: &str) -> String {
    value
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
}
