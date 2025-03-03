use anyhow::Context as _;
use data::config::Config;
use eframe::icon_data;
use egui::ViewportBuilder;

mod data;
#[cfg(test)]
mod test;
mod ui;
mod utils;

fn main() -> anyhow::Result<()> {
    utils::initialize_logger()?;
    let config_path = utils::get_config_path()?;
    let config = Config::load(&config_path)?;
    log::debug!("{config:?}");

    let viewport = ViewportBuilder::default()
        .with_title("Launcher")
        .with_app_id("rust-launcher")
        .with_inner_size(egui::vec2(900.0, 600.0))
        .with_icon(
            icon_data::from_png_bytes(include_bytes!("../res/icon.png")).unwrap_or_default(),
        );

    eframe::run_native(
        "Rust-Launcher",
        eframe::NativeOptions {
            viewport,
            centered: true,

            ..Default::default()
        },
        Box::new(|ctx| Ok(Box::new(ui::App::new(&ctx.egui_ctx, config)))),
    )
    .map_err(|e| anyhow::anyhow!(e.to_string()))
    .context("Failed to run native")
}
