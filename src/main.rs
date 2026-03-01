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

  /// Start with continuous Mandelbrot animation
  #[arg(long, alias = "auto-zoom")]
  animate: bool,

  /// Animation target FPS
  #[arg(long = "animate-fps", alias = "auto-fps", default_value_t = 18.0)]
  animate_fps: f64,

  /// Animation zoom multiplier per tick (higher means faster zoom)
  #[arg(
    long = "animate-zoom-factor",
    alias = "auto-zoom-factor",
    default_value_t = 1.02
  )]
  animate_zoom_factor: f64,

  /// Reset depth when scale grows above this threshold during animation
  #[arg(
    long = "animate-scale-ceiling",
    alias = "auto-scale-ceiling",
    alias = "auto-scale-floor",
    default_value_t = 1e13
  )]
  animate_scale_ceiling: f64,

  /// Disable deep-point cycling during animation
  #[arg(long = "animate-no-cycle", alias = "auto-no-cycle")]
  animate_no_cycle: bool,
}

fn main() {
  color_eyre::install().expect("panic");
  let args = Args::parse();

  if args.sixel {
    sixel::start_sixel_rendering(args.width, args.height, args.aspect_ratio);
  } else {
    let term = ratatui::init();
    let mut app = app::App::default();
    if args.animate {
      app.fractal.z.re = -0.743643887037151;
      app.fractal.z.im = 0.13182590420533;
      app.fractal.scale = 300.0;
      app.fractal.max_iterations = 180;
      app.configure_auto_zoom(
        args.animate_fps,
        args.animate_zoom_factor,
        args.animate_scale_ceiling,
        !args.animate_no_cycle,
      );
    }

    app.run(term).expect("App encountered an error");
    ratatui::restore();
  }
}
