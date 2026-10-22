use std::path::Path;

use crate::{compiler::model::document::Site, config::Config, error::QFError};

pub mod html;
pub mod rss;
pub mod text;

pub trait OutputFormatter {
    fn name(&self) -> &'static str;
    fn write(&self, site: &Site, config: &Config, stage: &Path) -> Result<(), QFError>;
}

pub fn outputs() -> Vec<Box<dyn OutputFormatter>> {
    return vec![
        Box::new(html::HTMLOutputFormatter),
        Box::new(text::TextOutputFormatter),
        Box::new(rss::RSSOutputFormatter),
    ];
}
