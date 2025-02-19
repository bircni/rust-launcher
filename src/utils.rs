use anyhow::Context as _;
use simple_home_dir::home_dir;
use std::path::PathBuf;

pub fn get_config_path() -> anyhow::Result<PathBuf> {
    let home_dir = home_dir().context("Failed to get home directory")?;
    let config_path = home_dir.join(".config").join("config.json");
    Ok(config_path)
}
