use crate::complex::Complex;
use crate::fractal::{Fractal, Set};
use crate::fractal_iterator::{
  BurningShipIterator, FractalIterator, MandelbrotIterator, PhoenixIterator,
};
use image::Rgb;
use rayon::prelude::*;

const ESCAPE_RADIUS_SQ: f64 = 4.0;
const SMOOTH_OFFSET: f64 = 1.0;
const LOG2: f64 = std::f64::consts::LN_2;

fn iterate_point(
  iterator: &dyn FractalIterator,
  mut z: Complex,
  c: Complex,
  max_iterations: u32,
  smooth: bool,
) -> f64 {
  let mut z_prev = Complex::new(0.0, 0.0);
  let mut i = 0;

  while z.abs_sq() <= ESCAPE_RADIUS_SQ && i < max_iterations {
    let temp_z = z;
    z = iterator.iterate(z, z_prev, c);
    z_prev = temp_z;
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

  let iterator: Box<dyn FractalIterator + Send + Sync> = match fractal.set {
    Set::Mandelbrot => Box::new(MandelbrotIterator {
      power: fractal.power,
    }),
    Set::BurningShip => Box::new(BurningShipIterator),
    Set::Julia => Box::new(MandelbrotIterator {
      power: fractal.power,
    }),
    Set::Phoenix => Box::new(PhoenixIterator {
      power: fractal.power,
      c: fractal.phoenix_c,
      p: fractal.phoenix_p,
    }),
  };

  (0..height)
    .into_par_iter()
    .map(|y| {
      (0..width)
        .map(|x| {
          let cx = left + x as f64 * vw / width as f64;
          let cy = top + y as f64 * vh / height as f64;
          let (z, c) = match fractal.set {
            Set::Mandelbrot | Set::BurningShip => (Complex::new(0.0, 0.0), Complex::new(cx, cy)),
            Set::Julia => (
              Complex::new(cx, cy),
              Complex::new(fractal.julia_c.re, fractal.julia_c.im),
            ),
            Set::Phoenix => {
              (Complex::new(cy, cx), Complex::new(0.0, 0.0)) // WTF is it rotated ??? TODO
            }
          };

          let iter = iterate_point(iterator.as_ref(), z, c, fractal.max_iterations, smooth);

          fractal.colorize(iter)
        })
        .collect()
    })
    .collect()
}
