use std::path::Path;

use crate::{compiler::model::document::Site, config::Config, output::OutputFormatter};

pub struct HTMLOutputFormatter;

impl OutputFormatter for HTMLOutputFormatter {
    fn name(&self) -> &'static str {
        "html"
    }

    fn write(&self, site: &Site, config: &Config, stage: &Path) -> Result<(), String> {
        Ok(())
    }
}

fn html_escape(value: &str) -> String {
    value
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#39;")
}
