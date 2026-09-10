use std::path::Path;

use crate::{compiler::model::document::Site, config::Config};

pub mod html;
pub mod text;

pub trait OutputFormatter {
    fn name(&self) -> &'static str;
    fn write(&self, site: &Site, config: &Config, stage: &Path) -> Result<(), String>;
}
