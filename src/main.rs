mod animation;
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
  #[arg(long, requires = "sixel", default_value_t = 600)]
  width: u32,

  /// Height of the Sixel image (default: 600)
  /// Only used if --sixel is present
  #[arg(long, requires = "sixel", default_value_t = 600)]
  height: u32,

  /// Aspect ratio of the terminal (default: 1.0 for 1:2 fonts)
  /// Only used if --sixel is present
  #[arg(short = 'r', requires = "sixel", default_value_t = 1.0)]
  aspect_ratio: f64,
}

fn main() {
  color_eyre::install().expect("panic");
  let args = Args::parse();

  if args.sixel {
    sixel::start_sixel_rendering(args.width, args.height, args.aspect_ratio);
  } else {
    let term = ratatui::init();
    app::App::default()
      .run(term)
      .expect("App encountered an error");
    ratatui::restore();
  }
}
