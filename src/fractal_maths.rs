use crate::fractal::{Fractal, Set};
use crate::palettes::{PALETTES, PaletteFn};
use image::Rgb;
use rayon::prelude::*;
use crate::complex::Complex;

const ESCAPE_RADIUS_SQ: f64 = 4.0;
const SMOOTH_OFFSET: f64 = 1.0;
const LOG2: f64 = std::f64::consts::LN_2;

trait FractalIterator: Send + Sync {
  fn iterate(&self, z: Complex, c: Complex) -> Complex;
}

struct MandelbrotIterator { power: f64 }
struct BurningShipIterator;

impl FractalIterator for MandelbrotIterator {
  fn iterate(&self, z: Complex, c: Complex) -> Complex {
    if self.power == 2.0 {
      z.square().add(c)
    } else {
      Complex::polar(self.power)(z).add(c)
    }
  }
}

impl FractalIterator for BurningShipIterator {
  fn iterate(&self, z: Complex, c: Complex) -> Complex {
    let abs_z = z.abs();
    Complex {
      re: abs_z.re * abs_z.re - abs_z.im * abs_z.im,
      im: 2.0 * abs_z.re * abs_z.im,
    }.add(c)
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
  z: Complex,
  c: Complex,
  max_iterations: u32,
  smooth: bool,
) -> f64 {
  let mut z = z;
  let mut i = 0;

  while z.abs_sq() <= ESCAPE_RADIUS_SQ && i < max_iterations {
    z = iterator.iterate(z, c);
    i += 1;
  }

  if smooth && i < max_iterations {
    let log_zn = z.abs_sq().sqrt().ln().ln();
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
  let left = fractal.z.re - vw / 2.0;
  let top = fractal.z.im - vh / 2.0;

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
          let (z, c) = match fractal.set {
            Set::Mandelbrot | Set::BurningShip => {
              let cx = left + x as f64 * vw / width as f64;
              let cy = top + y as f64 * vh / height as f64;
              (Complex::new(0.0, 0.0), Complex::new(cx, cy))
            }
            Set::Julia => {
              let zx = left + x as f64 * vw / width as f64;
              let zy = top + y as f64 * vh / height as f64;
              (
                Complex::new(zx, zy),
                Complex::new(fractal.julia_constant.re, fractal.julia_constant.im),
              )
            }
            Set::Phoenix => panic!("Phoenix not implemented"),
          };

          let iter = iterate_point(iterator.as_ref(), z, c, fractal.max_iterations, smooth);

          colorize(iter, fractal.max_iterations, PALETTES[fractal.current_palette])
        })
        .collect()
    })
    .collect()
}