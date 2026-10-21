use serde::Deserialize;
use std::{
    fs,
    path::{Path, PathBuf},
};

use crate::error::QFError;

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    pub title: String,
    pub base_url: String,
    pub stylesheet: Option<String>,
    pub timezone: String,

    #[serde(default = "default_language")]
    pub language: String,

    #[serde(default = "default_content_dir")]
    pub content_dir: PathBuf,

    #[serde(default = "default_output_dir")]
    pub output_dir: PathBuf,

    #[serde(default = "default_static_dir")]
    pub static_dir: PathBuf,
}

fn default_language() -> String {
    "en".to_string()
}

fn default_content_dir() -> PathBuf {
    PathBuf::from("content")
}

fn default_output_dir() -> PathBuf {
    PathBuf::from("dist")
}

fn default_static_dir() -> PathBuf {
    PathBuf::from("static")
}

impl Config {
    pub fn load(root: &Path) -> Result<Self, QFError> {
        let path = root.join("qf.config.json");
        let raw = fs::read_to_string(&path)?;
        let config: Config = serde_json::from_str(&raw)
            .map_err(|e| QFError::Message(format!("Failed to parse config file: {}", e)))?;
        Ok(config)
    }
}
