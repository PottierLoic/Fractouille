mod app;
mod command;
mod complex;
mod export;
mod fractal;
mod palette;
mod ui;

use color_eyre::Result;

fn main() -> Result<()> {
  color_eyre::install()?;
  let term = ratatui::init();
  let res = app::App::default().run(term);
  ratatui::restore();
  res
}
