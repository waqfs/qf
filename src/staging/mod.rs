use std::{
    fs,
    path::{Path, PathBuf},
    time::{SystemTime, UNIX_EPOCH},
};

use crate::config::Config;

pub fn init_stage(root: &Path, config: &Config) -> Result<PathBuf, String> {
    let uid = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis();
    let path = root.join(format!(".{}.staging-{uid}", config.output_dir.display()));
    if path.exists() {
        fs::remove_dir_all(&path)
            .map_err(|e| format!("Staging directory exists and failed to be removed. {e}"))?;
    }
    fs::create_dir_all(&path)
        .map_err(|e| format!("Staging directory failed to be created. {e}"))?;
    Ok(path)
}

pub fn commit_stage(root: &Path, config: &Config, path: &Path) -> Result<(), String> {
    let out_path = root.join(&config.output_dir);
    if out_path.exists() {
        fs::remove_dir_all(&out_path)
            .map_err(|e| format!("Output directory failed to be removed. {e}"))?;
    }
    fs::rename(path, out_path)
        .map_err(|e| format!("Staging directory failed to be renamed to output directory. {e}"))?;
    Ok(())
}

pub fn insert_static(root: &Path, config: &Config, stage: &Path) -> Result<(), String> {
    let src = root.join(&config.static_dir);
    if src.exists() {
        copy_dir(&src, stage)?;
    }
    Ok(())
}

fn copy_dir(src: &Path, dest: &Path) -> Result<(), String> {
    fs::create_dir_all(dest)
        .map_err(|e| format!("Copy destination directory could not be created. {e}"))?;
    for file in fs::read_dir(src)
        .map_err(|e| format!("Could not read file in directory {}: {e}", src.display()))?
    {
        let file =
            file.map_err(|e| format!("An error occurred reading file {}: {e}", src.display()))?;
        let from = file.path();
        let to = dest.join(file.file_name());
        if from.is_dir() {
            copy_dir(&from, &to)?;
        } else {
            fs::copy(from, to)
                .map_err(|e| format!("An error occurred copying files to directory: {e}"))?;
        }
    }
    Ok(())
}
