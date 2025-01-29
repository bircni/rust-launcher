use anyhow::Context as _;
use simple_home_dir::home_dir;
use std::path::PathBuf;
use tray_icon::{menu::Menu, TrayIconBuilder};

pub fn get_config_path() -> anyhow::Result<PathBuf> {
    let home_dir = home_dir().context("Failed to get home directory")?;
    let config_path = home_dir.join(".config").join("config.json");
    Ok(config_path)
}

/// From <https://github.com/tauri-apps/tray-icon/blob/dev/examples/egui.rs>
pub fn load_icon() -> anyhow::Result<tray_icon::Icon> {
    let logo = include_bytes!("../res/logo.png");
    let image = image::load_from_memory(logo)?.into_rgba8();
    let (width, height) = image.dimensions();
    let rgba = image.into_raw();

    tray_icon::Icon::from_rgba(rgba, width, height).context("Failed to load icon")
}

pub fn create_tray_icon(icon: tray_icon::Icon) -> anyhow::Result<tray_icon::TrayIcon> {
    let menu = Menu::new();
    let tray_icon = TrayIconBuilder::new()
        .with_icon(icon)
        .with_menu(Box::new(menu))
        .build()?;
    Ok(tray_icon)
}
