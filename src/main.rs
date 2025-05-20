use crate::app::App;

#[macro_use]
mod macros;

pub mod app;
pub mod event;
mod md;
mod state;
mod tabs;
pub mod ui;

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    let terminal = ratatui::init();
    let result = App::new().run(terminal).await;
    ratatui::restore();
    result
}
