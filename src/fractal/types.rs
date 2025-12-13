use crate::complex::Complex;
use crate::palette::{Palette, default_palettes};

#[derive(Debug, Clone, PartialEq)]
pub enum Set {
  Mandelbrot,
  Julia,
  BurningShip,
  Phoenix,
}

#[derive(Debug, Clone)]
pub struct Fractal {
  pub z: Complex,
  pub scale: f64,
  pub max_iterations: u32,
  pub palette: Vec<Palette>,
  pub current_palette: usize,
  pub set: Set,
  pub julia_c: Complex,
  pub phoenix_c: Complex,
  pub phoenix_p: Complex,
  pub power: f64,
}

impl Default for Fractal {
  fn default() -> Self {
    Self {
      z: Complex::new(-0.5, 0.0),
      scale: 1.0,
      max_iterations: 100,
      palette: default_palettes(),
      current_palette: 0,
      set: Set::Mandelbrot,
      julia_c: Complex::new(-0.5125, 0.5213),
      phoenix_c: Complex::new(0.0, 0.0),
      phoenix_p: Complex::new(-0.5, 0.0),
      power: 2.0,
    }
  }
}
