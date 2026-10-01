use std::{fs, ops::Add, path::Path};

use chrono::Local;

use crate::{
    compiler::model::{
        document::Site,
        site::{Document, DocumentType},
    },
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
    body.push_str("<?xml version=\"1.0\" encoding=\"UTF-8\"?>");
    body.push_str("<rss version=\"2.0\">\n");
    body.push_str("<channel>\n");
    body.push_str(&format!("<title>{}</title>\n", xml_escape(&config.title)));
    body.push_str(&format!("<link>{}</link>\n", xml_escape(&config.base_url)));
    body.push_str(&format!(
        "<description>{}</description>\n",
        match &document.metadata.summary {
            Some(summary) => xml_escape(summary),
            None => "No description provided".to_string(),
        }
    ));
    // body.push_str(&format!("<pubDate></pubDate>"));
    body.push_str(&format!(
        "<lastBuildDate>{}</lastBuildDate>\n",
        Local::now().to_rfc2822()
    ));
    body.push_str(&format!(
        "<category>{}</category>\n",
        match &document.metadata.doc_type {
            DocumentType::Article => "Articles",
            DocumentType::Project => "Projects",
            DocumentType::Page => "Miscellaneous",
        }
    ));
    body.push_str("<generator>qf (https://github.com/waqfs/qf)</generator>\n");
    body.push_str("<docs>https://www.rssboard.org/rss-specification</docs>\n");
    write_items(site, &document.metadata.doc_type, config, &mut body);
    body.push_str("</channel>\n");
    body.push_str("</rss>");
    body
}

fn write_items(site: &Site, doc_type: &DocumentType, config: &Config, output: &mut String) {
    let documents = match doc_type {
        DocumentType::Article => &site.articles,
        DocumentType::Project => &site.projects,
        DocumentType::Page => return,
    };
    let doc_type_str = match doc_type {
        DocumentType::Article => "article",
        DocumentType::Project => "project",
        DocumentType::Page => "page",
    };
    for index in documents {
        let document = &site.documents[*index];
        output.push_str("<item>");
        output.push_str(&format!("<title>{}</title>", document.metadata.title));
        output.push_str(&format!(
            "<link>{}</link>",
            config.base_url.clone().add(&document.route)
        ));
        if let Some(summary) = &document.metadata.summary {
            output.push_str(&format!("<description>{}</description>", summary));
        }
        if let Some(author) = &document.metadata.author {
            output.push_str(&format!("<author>{}</author>", author));
        }
        if let Some(date) = &document.metadata.date {
            output.push_str(&format!("<pubDate>{}</pubDate>", date.to_rfc2822()));
        }
        if let Some(handle) = &document.metadata.handle {
            output.push_str(&format!(
                "<guid isPermaLink=\"false\">qf:{}:{}</guid>",
                doc_type_str, handle
            ));
        }
        output.push_str("</item>\n");
    }
}

fn xml_escape(value: &str) -> String {
    value
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
}
