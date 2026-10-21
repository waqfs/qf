use std::{
    fs,
    path::{Path, PathBuf},
};

use crate::{config::Config, error::QFError};

#[derive(Debug, Clone)]
pub struct PortfolioFile {
    pub path: PathBuf,
    pub raw: String,
}

pub fn get_raw(path: &Path) -> Result<PortfolioFile, QFError> {
    Ok(PortfolioFile {
        path: path.to_path_buf(),
        raw: fs::read_to_string(path)
            .map_err(|e| QFError::Message(format!("Failed to read project file {}", e)))?,
    })
}

pub fn get_portfolio_files(root: &Path, config: &Config) -> Result<Vec<PathBuf>, QFError> {
    let directory = root.join(&config.content_dir);
    let mut files = Vec::new();
    iter(&directory, &mut files)?;
    Ok(files)
}

fn iter(path: &Path, paths: &mut Vec<PathBuf>) -> Result<(), QFError> {
    for entry in fs::read_dir(path)
        .map_err(|e| QFError::Message(format!("Failed to read directory {}", e)))?
    {
        let file =
            entry.map_err(|e| QFError::Message(format!("Failed to read directory file {}", e)))?;
        let path = file.path();
        if path.is_dir() {
            iter(&path, paths)?;
        } else if path.extension().and_then(|v| v.to_str()) == Some("qf") {
            paths.push(path);
        }
    }
    Ok(())
}
