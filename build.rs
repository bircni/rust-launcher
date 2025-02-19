use std::env;

use winresource::WindowsResource;

fn main() -> anyhow::Result<()> {
    let target = env::var("CARGO_CFG_TARGET_OS")?;
    if target == "windows" {
        let mut res = WindowsResource::new();
        res.set_icon("./res/icon.ico");
        res.compile()?;
    }
    Ok(())
}
