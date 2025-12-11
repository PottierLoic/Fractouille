use crate::complex::Complex;

pub trait FractalIterator: Send + Sync {
  fn iterate(&self, z: Complex, z_prev: Complex, c: Complex) -> Complex;
}

pub struct MandelbrotIterator {
  pub power: f64,
}

pub struct BurningShipIterator;

pub struct PhoenixIterator {
  pub power: f64,
  pub c: Complex,
  pub p: Complex,
}

impl FractalIterator for MandelbrotIterator {
  fn iterate(&self, z: Complex, _z_prev: Complex, c: Complex) -> Complex {
    if self.power == 2.0 {
      z.square().add(c)
    } else {
      Complex::polar(self.power)(z).add(c)
    }
  }
}

impl FractalIterator for BurningShipIterator {
  fn iterate(&self, z: Complex, _z_prev: Complex, c: Complex) -> Complex {
    let abs_z = z.abs();
    Complex {
      re: abs_z.re * abs_z.re - abs_z.im * abs_z.im,
      im: 2.0 * abs_z.re * abs_z.im,
    }
    .add(c)
  }
}

impl FractalIterator for PhoenixIterator {
  fn iterate(&self, z: Complex, z_prev: Complex, _c: Complex) -> Complex {
    let powered_z = if self.power == 2.0 {
      z.square()
    } else {
      Complex::polar(self.power)(z)
    };
    powered_z.add(self.c).add(self.p.mul(z_prev))
  }
}
