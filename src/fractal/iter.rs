use crate::complex::Complex;
use crate::fractal::constants::ESCAPE_RADIUS_SQ;

pub fn iterate_mandelbrot(mut z: Complex, c: Complex, max_iter: u32, power: f64) -> (u32, Complex) {
  for i in 0..max_iter {
    if z.abs_sq() > ESCAPE_RADIUS_SQ {
      return (i, z);
    }
    if power == 2.0 {
      z = z.square().add(c);
    } else {
      z = Complex::polar(power)(z).add(c);
    }
  }
  (max_iter, z)
}

pub fn iterate_julia(mut z: Complex, c: Complex, max_iter: u32, power: f64) -> (u32, Complex) {
  for i in 0..max_iter {
    if z.abs_sq() > ESCAPE_RADIUS_SQ {
      return (i, z);
    }
    if power == 2.0 {
      z = z.square().add(c);
    } else {
      z = Complex::polar(power)(z).add(c);
    }
  }
  (max_iter, z)
}

pub fn iterate_burningship(mut z: Complex, c: Complex, max_iter: u32) -> (u32, Complex) {
  for i in 0..max_iter {
    if z.abs_sq() > ESCAPE_RADIUS_SQ {
      return (i, z);
    }
    z = z.abs().square().add(c);
  }
  (max_iter, z)
}

pub fn iterate_phoenix(
  mut z: Complex,
  c: Complex,
  p: Complex,
  max_iter: u32,
  power: f64,
) -> (u32, Complex) {
  let mut z_prev = Complex::new(0.0, 0.0);
  for i in 0..max_iter {
    if z.abs_sq() > ESCAPE_RADIUS_SQ {
      return (i, z);
    }

    let z_next = if power == 2.0 {
      z.square().add(c).add(z_prev.mul(p))
    } else {
      Complex::polar(power)(z).add(c).add(z_prev.mul(p))
    };

    z_prev = z;
    z = z_next;
  }
  (max_iter, z)
}
