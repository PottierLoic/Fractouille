use crate::fractal::{Fractal, Set};
use crate::palettes::{PALETTES, PaletteFn};
use image::Rgb;
use rayon::prelude::*;

const ESCAPE_RADIUS_SQ: f64 = 4.0;
const SMOOTH_OFFSET: f64 = 1.0;
const LOG2: f64 = std::f64::consts::LN_2;

trait FractalIterator: Send + Sync {
  fn iterate(&self, zx: f64, zy: f64, cx: f64, cy: f64) -> (f64, f64);
}

struct MandelbrotIterator { power: f64 }
struct BurningShipIterator;

impl FractalIterator for MandelbrotIterator {
  fn iterate(&self, zx: f64, zy: f64, cx: f64, cy: f64) -> (f64, f64) {
    if self.power == 2.0 {
      let new_zx = zx * zx - zy * zy + cx;
      let new_zy = 2.0 * zx * zy + cy;
      (new_zx, new_zy)
    } else {
      let r = (zx * zx + zy * zy).sqrt();
      let theta = zy.atan2(zx);
      let r_pow = r.powf(self.power);
      let angle = self.power * theta;
      (r_pow * angle.cos() + cx, r_pow * angle.sin() + cy)
    }
  }
}

impl FractalIterator for BurningShipIterator {
  fn iterate(&self, zx: f64, zy: f64, cx: f64, cy: f64) -> (f64, f64) {
    let abs_zx = zx.abs();
    let abs_zy = zy.abs();
    let new_zx = abs_zx * abs_zx - abs_zy * abs_zy + cx;
    let new_zy = 2.0 * abs_zx * abs_zy + cy;
    (new_zx, new_zy)
  }
}

fn colorize(iter: f64, max_iter: u32, palette: PaletteFn) -> Rgb<u8> {
  let (r, g, b) = if iter >= max_iter as f64 {
    (0, 0, 0)
  } else {
    palette(iter / max_iter as f64)
  };
  Rgb([r, g, b])
}

fn iterate_point(
  iterator: &dyn FractalIterator,
  zx: f64,
  zy: f64,
  cx: f64,
  cy: f64,
  max_iterations: u32,
  smooth: bool,
) -> f64 {
  let mut zx = zx;
  let mut zy = zy;
  let mut i = 0;

  while zx * zx + zy * zy <= ESCAPE_RADIUS_SQ && i < max_iterations {
    (zx, zy) = iterator.iterate(zx, zy, cx, cy);
    i += 1;
  }

  if smooth && i < max_iterations {
    let log_zn = (zx * zx + zy * zy).sqrt().ln().ln();
    i as f64 + SMOOTH_OFFSET - log_zn / LOG2
  } else {
    i as f64
  }
}
pub fn generate_image(
  fractal: &Fractal,
  width: u32,
  height: u32,
  smooth: bool,
) -> Vec<Vec<Rgb<u8>>> {
  let aspect = width as f64 / height as f64;
  let vw = 3.5 / fractal.scale;
  let vh = vw / aspect;
  let left = fractal.z.0 - vw / 2.0;
  let top = fractal.z.1 - vh / 2.0;

  let iterator: Box<dyn FractalIterator> = match fractal.set {
    Set::Mandelbrot => Box::new(MandelbrotIterator { power: fractal.power }),
    Set::BurningShip => Box::new(BurningShipIterator),
    Set::Julia => Box::new(MandelbrotIterator { power: fractal.power }),
    Set::Phoenix => panic!("Phoenix not implemented"),
  };

  (0..height)
    .into_par_iter()
    .map(|y| {
      (0..width)
        .map(|x| {
          let (zx, zy, cx, cy) = match fractal.set {
            Set::Mandelbrot | Set::BurningShip => {
              let cx = left + x as f64 * vw / width as f64;
              let cy = top + y as f64 * vh / height as f64;
              (0.0, 0.0, cx, cy)
            }
            Set::Julia => {
              let zx = left + x as f64 * vw / width as f64;
              let zy = top + y as f64 * vh / height as f64;
              (zx, zy, fractal.julia_constant.0, fractal.julia_constant.1)
            }
            Set::Phoenix => panic!("Phoenix not implemented"),
          };

          let iter = iterate_point(
            iterator.as_ref(),
            zx,
            zy,
            cx,
            cy,
            fractal.max_iterations,
            smooth,
          );

          colorize(
            iter,
            fractal.max_iterations,
            PALETTES[fractal.current_palette],
          )
        })
        .collect()
    })
    .collect()
}