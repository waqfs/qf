use std::path::Path;

use crate::{compiler::model::document::Site, config::Config};

pub mod html;
pub mod text;
pub mod rss;

pub trait OutputFormatter {
    fn name(&self) -> &'static str;
    fn write(&self, site: &Site, config: &Config, stage: &Path) -> Result<(), String>;
}

pub fn outputs() -> Vec<Box<dyn OutputFormatter>> {
    return vec![
        Box::new(html::HTMLOutputFormatter),
        Box::new(text::TextOutputFormatter),
        Box::new(rss::RSSOutputFormatter),
    ];
}
