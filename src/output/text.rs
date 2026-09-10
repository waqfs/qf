use std::path::Path;

use crate::{compiler::model::document::Site, config::Config, output::OutputFormatter};

pub struct TextOutputFormatter;

impl OutputFormatter for TextOutputFormatter {
    fn name(&self) -> &'static str {
        "text"
    }

    fn write(&self, site: &Site, config: &Config, stage: &Path) -> Result<(), String> {
        Ok(())
    }
}
