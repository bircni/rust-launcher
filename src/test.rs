#![expect(clippy::unwrap_used, reason = "this is a test")]
use crate::utils;

#[test]
fn test_get_config_path() {
    utils::get_config_path().unwrap();
}

#[test]
fn test_load_icon() {
    utils::load_icon().unwrap();
}

#[test]
fn test_create_tray_icon() {
    let icon = utils::load_icon().unwrap();
    utils::create_tray_icon(icon).unwrap();
}
