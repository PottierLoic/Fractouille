mod app;
mod command;
mod complex;
mod export;
mod fractal;
mod palette;
mod sixel;
mod ui;

use clap::Parser;

#[derive(Parser)]
struct Args {
  /// Render using SIXEL output
  #[arg(long)]
  sixel: bool,
}

fn main() {
  color_eyre::install().expect("panic");
  let args = Args::parse();

  if args.sixel {
    sixel::start_sixel_rendering();
  } else {
    let term = ratatui::init();
    app::App::default()
      .run(term)
      .expect("App encountered an error");
    ratatui::restore();
  }
}
