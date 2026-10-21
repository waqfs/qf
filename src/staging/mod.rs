use std::{
    fs,
    path::{Path, PathBuf},
    time::{SystemTime, UNIX_EPOCH},
};

use crate::{config::Config, error::QFError};

pub fn init_stage(root: &Path, config: &Config) -> Result<PathBuf, QFError> {
    let uid = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis();
    let path = root.join(format!(".{}.staging-{uid}", config.output_dir.display()));
    if path.exists() {
        fs::remove_dir_all(&path)?;
    }
    fs::create_dir_all(&path)?;
    Ok(path)
}

pub fn commit_stage(root: &Path, config: &Config, path: &Path) -> Result<(), QFError> {
    let out_path = root.join(&config.output_dir);
    if out_path.exists() {
        fs::remove_dir_all(&out_path)?;
    }
    fs::rename(path, out_path)?;
    Ok(())
}

pub fn insert_static(root: &Path, config: &Config, stage: &Path) -> Result<(), QFError> {
    let src = root.join(&config.static_dir);
    if src.exists() {
        copy_dir(&src, stage)?;
    }
    Ok(())
}

fn copy_dir(src: &Path, dest: &Path) -> Result<(), QFError> {
    fs::create_dir_all(dest)?;
    for file in fs::read_dir(src)? {
        let file = file?;
        let from = file.path();
        let to = dest.join(file.file_name());
        if from.is_dir() {
            copy_dir(&from, &to)?;
        } else {
            fs::copy(from, to)?;
        }
    }
    Ok(())
}
