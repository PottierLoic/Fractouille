use crate::complex::Complex;
use crate::fractal::constants::{LOG2, SMOOTH_OFFSET};
use crate::fractal::iter::{
  iterate_burningship, iterate_julia, iterate_mandelbrot, iterate_phoenix,
};
use crate::fractal::{Fractal, Set};
use image::Rgb;
use rayon::iter::IntoParallelIterator;
use rayon::iter::ParallelIterator;

impl Fractal {
  pub fn render_frame(&self, width: u32, height: u32, smooth: bool) -> Vec<Vec<Rgb<u8>>> {
    let aspect = width as f64 / height as f64;
    let vw = 3.5 / self.scale;
    let vh = vw / aspect;
    let left = self.z.re - vw / 2.0;
    let top = self.z.im - vh / 2.0;

    (0..height)
      .into_par_iter()
      .map(|y| {
        (0..width)
          .map(|x| {
            let cx = left + x as f64 * vw / width as f64;
            let cy = top + y as f64 * vh / height as f64;

            let (z0, c0) = match self.set {
              Set::Mandelbrot | Set::BurningShip => (Complex::new(0.0, 0.0), Complex::new(cx, cy)),
              Set::Julia => (Complex::new(cx, cy), self.julia_c),
              Set::Phoenix => (Complex::new(0.0, 0.0), Complex::new(cy, cx)),
            };

            let (iter, final_z) = match self.set {
              Set::Mandelbrot => {
                if self.power == 2.0 && in_bulb(c0) {
                  (255, z0)
                } else {
                  iterate_mandelbrot(z0, c0, self.max_iterations, self.power)
                }
              }
              Set::Julia => iterate_julia(z0, c0, self.max_iterations, self.power),
              Set::BurningShip => iterate_burningship(z0, c0, self.max_iterations),
              Set::Phoenix => {
                iterate_phoenix(z0, c0, self.phoenix_p, self.max_iterations, self.power)
              }
            };

            let value = if smooth && iter < self.max_iterations {
              let log_zn = final_z.abs_sq().sqrt().ln().ln();
              iter as f64 + SMOOTH_OFFSET - log_zn / LOG2
            } else {
              iter as f64
            };

            // Return RGB
            self.colorize(value)
          })
          .collect()
      })
      .collect()
  }

  fn colorize(&self, iter: f64) -> Rgb<u8> {
    if iter >= self.max_iterations as f64 {
      return Rgb([0, 0, 0]);
    }

    let palette = &self.palette[self.current_palette];
    let (r, g, b) = palette.eval(iter / palette.cycle_speed);
    Rgb([r, g, b])
  }
}

// Check if the point is in the main cardioid or period 2 bulb
// work only for power=2 mandelbrot set
#[inline(always)]
fn in_bulb(c: Complex) -> bool {
  let x = c.re - 0.25;
  let y = c.im;
  let q = x * x + y * y;
  if q * (q + x) <= 0.25 * y * y {
    return true;
  }

  let dx = c.re + 1.0;
  if dx * dx + c.im * c.im <= 0.0625 {
    return true;
  }

  false
}
