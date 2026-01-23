mod app;
mod command;
mod complex;
mod export;
mod fractal;
mod gui;
mod palette;
mod sixel;
mod ui;

use clap::Parser;

#[derive(Parser)]
struct Args {
  /// Render using SIXEL output
  #[arg(long)]
  sixel: bool,

  /// Render using eframe GUI
  #[arg(long)]
  gui: bool,
}

fn main() {
  color_eyre::install().expect("panic");
  let args = Args::parse();

  if args.sixel {
    sixel::start_sixel_rendering();
    // TODO : Error handling
  } else if args.gui {
    gui::start_gui_app().expect("GUI app encountered an error");
  } else {
    let term = ratatui::init();
    app::App::default()
      .run(term)
      .expect("Ratatui app encountered an error");
    ratatui::restore();
  }
}
