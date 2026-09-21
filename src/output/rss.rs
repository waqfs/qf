use std::path::Path;

use crate::{compiler::model::document::Site, config::Config, output::OutputFormatter};

pub struct RSSOutputFormatter;

impl OutputFormatter for RSSOutputFormatter {
    fn name(&self) -> &'static str {
        "rss"
    }

    fn write(&self, site: &Site, config: &Config, stage: &Path) -> Result<(), String> {
        Ok(())
    }
}

fn xml_escape(value: &str) -> String {
    value
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
}
