use crate::fractal::Set;
use crate::palettes::PaletteFn;
use crate::utils::color_to_rgb;
use image::Rgb;
use ratatui::style::Color;

pub trait FractalColorizer<T> {
  fn colorize(iter: f64, max_iter: u32, palette: PaletteFn) -> T;
}

impl FractalColorizer<Color> for Color {
  fn colorize(iter: f64, max_iter: u32, palette: PaletteFn) -> Self {
    if iter >= max_iter as f64 {
      Color::Black
    } else {
      palette(iter / max_iter as f64)
    }
  }
}

impl FractalColorizer<Rgb<u8>> for Rgb<u8> {
  fn colorize(iter: f64, max_iter: u32, palette: PaletteFn) -> Self {
    let color = if iter >= max_iter as f64 {
      Color::Black
    } else {
      palette(iter / max_iter as f64)
    };
    let (r, g, b) = color_to_rgb(&color);
    Rgb([r, g, b])
  }
}

fn iterate_point(
  set: &Set,
  mut zx: f64,
  mut zy: f64,
  cx: f64,
  cy: f64,
  max_iterations: u32,
) -> (u32, f64, f64) {
  let mut i = 0;
  match set {
    Set::BurningShip => {
      while zx * zx + zy * zy <= 4.0 && i < max_iterations {
        let abs_zx = zx.abs();
        let abs_zy = zy.abs();
        let tmp = abs_zx * abs_zx - abs_zy * abs_zy + cx;
        zy = 2.0 * abs_zx * abs_zy + cy;
        zx = tmp;
        i += 1;
      }
    }
    _ => {
      while zx * zx + zy * zy <= 4.0 && i < max_iterations {
        let tmp = zx * zx - zy * zy + cx;
        zy = 2.0 * zx * zy + cy;
        zx = tmp;
        i += 1;
      }
    }
  }

  (i, zx, zy)
}

pub fn iterate_point_raw(
  set: &Set,
  zx: f64,
  zy: f64,
  cx: f64,
  cy: f64,
  max_iterations: u32,
) -> u32 {
  let (i, _, _) = iterate_point(set, zx, zy, cx, cy, max_iterations);
  i
}

pub fn iterate_point_smooth(
  set: &Set,
  zx: f64,
  zy: f64,
  cx: f64,
  cy: f64,
  max_iterations: u32,
) -> f64 {
  let (i, zx, zy) = iterate_point(set, zx, zy, cx, cy, max_iterations);
  if i < max_iterations {
    let log_zn = (zx * zx + zy * zy).sqrt().ln().ln();
    i as f64 + 1.0 - log_zn / std::f64::consts::LN_2
  } else {
    i as f64
  }
}

pub fn generate_image<T: Clone, I: Fn(f64, f64, f64, f64) -> f64, C: Fn(f64) -> T>(
  set: &Set,
  w: usize,
  h: usize,
  center_x: f64,
  center_y: f64,
  scale: f64,
  real: f64,
  imag: f64,
  iterate_fn: I,
  colorize: C,
) -> Vec<Vec<T>> {
  let aspect = w as f64 / h as f64;
  let vw = 3.5 / scale;
  let vh = vw / aspect;
  let left = center_x - vw / 2.0;
  let top = center_y - vh / 2.0;

  (0..h)
    .map(|y| {
      (0..w)
        .map(|x| {
          let (zx, zy, cx, cy) = match set {
            Set::Mandelbrot | Set::BurningShip => {
              let cx = left + x as f64 * vw / w as f64;
              let cy = top + y as f64 * vh / h as f64;
              (0.0, 0.0, cx, cy)
            }
            Set::Julia => {
              let zx = left + x as f64 * vw / w as f64;
              let zy = top + y as f64 * vh / h as f64;
              (zx, zy, real, imag)
            }
          };

          let iter = iterate_fn(zx, zy, cx, cy);
          colorize(iter)
        })
        .collect()
    })
    .collect()
}
