#[cfg(not(target_os = "linux"))]
use std::{cell::RefCell, rc::Rc};

use anyhow::Context as _;
use data::config::Config;
use egui::ViewportBuilder;

mod data;
#[cfg(test)]
mod test;
mod ui;
mod utils;

fn main() -> anyhow::Result<()> {
    let config_path = utils::get_config_path()?;
    let config = Config::load(&config_path)?;
    println!("{config:?}");

    let viewport = ViewportBuilder::default()
        .with_title("Launcher")
        .with_app_id("rust-launcher")
        .with_inner_size(egui::vec2(900.0, 600.0));

    let icon = utils::load_icon()?;

    // Since egui uses winit under the hood and doesn't use gtk on Linux, and we need gtk for
    // the tray icon to show up, we need to spawn a thread
    // where we initialize gtk and create the tray_icon
    #[cfg(target_os = "linux")]
    std::thread::spawn(|| {
        gtk::init().expect("Failed to initialize gtk");
        let _tray_icon = utils::create_tray_icon(icon).expect("Failed to create tray icon");

        gtk::main();
    });

    #[cfg(not(target_os = "linux"))]
    let tray_icon = Rc::new(RefCell::new(None));
    #[cfg(not(target_os = "linux"))]
    let tray = utils::create_tray_icon(icon)?;

    eframe::run_native(
        "Rust-Launcher",
        eframe::NativeOptions {
            viewport,
            centered: true,
            ..Default::default()
        },
        Box::new(|ctx| {
            #[cfg(not(target_os = "linux"))]
            {
                tray_icon.borrow_mut().replace(tray);
            }
            Ok(Box::new(ui::App::new(&ctx.egui_ctx, config)))
        }),
    )
    .map_err(|e| anyhow::anyhow!(e.to_string()))
    .context("Failed to run native")
}
