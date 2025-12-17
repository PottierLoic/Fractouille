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

  /// Width of the Sixel image (default: 600)
  /// Only used if --sixel is present
  #[arg(short = 'w', requires = "sixel", default_value_t = 600)]
  width: u32,

  /// Height of the Sixel image (default: 600)
  /// Only used if --sixel is present
  #[arg(short = 'h', requires = "sixel", default_value_t = 600)]
  height: u32,
}

fn main() {
  color_eyre::install().expect("panic");
  let args = Args::parse();

  if args.sixel {
    sixel::start_sixel_rendering(args.width, args.height);
  } else {
    let term = ratatui::init();
    app::App::default()
      .run(term)
      .expect("App encountered an error");
    ratatui::restore();
  }
}
