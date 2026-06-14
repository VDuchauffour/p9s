pub mod app;
pub mod client;
pub mod config;
pub mod event;
pub mod tui;
pub mod ui;

pub fn run() -> anyhow::Result<()> {
    let _ = app::App::new()?;
    Ok(())
}
