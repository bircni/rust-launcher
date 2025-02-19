use anyhow::Context as _;
use log::LevelFilter;
use simple_home_dir::home_dir;
use simplelog::{ColorChoice, ConfigBuilder, TerminalMode};
use std::path::PathBuf;

pub fn get_config_path() -> anyhow::Result<PathBuf> {
    let home_dir = home_dir().context("Failed to get home directory")?;
    let config_path = home_dir.join(".config").join("config.json");
    Ok(config_path)
}

pub fn initialize_logger() -> anyhow::Result<()> {
    simplelog::TermLogger::init(
        #[cfg(debug_assertions)]
        LevelFilter::max(),
        #[cfg(not(debug_assertions))]
        LevelFilter::Info,
        ConfigBuilder::new()
            // suppress all logs from dependencies
            .add_filter_allow_str("rust_launcher")
            .build(),
        TerminalMode::Mixed,
        ColorChoice::Auto,
    )
    .context("Failed to initialize logger")
}
