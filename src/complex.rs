#[derive(Clone, Copy, Debug)]
pub struct Complex {
  pub re: f64,
  pub im: f64,
}

impl Complex {
  pub fn new(re: f64, im: f64) -> Self {
    Complex { re, im }
  }

  pub fn abs_sq(&self) -> f64 {
    self.re * self.re + self.im * self.im
  }

  pub fn add(self, other: Complex) -> Complex {
    Complex {
      re: self.re + other.re,
      im: self.im + other.im,
    }
  }

  pub fn mul(self, other: Complex) -> Complex {
    Complex {
      re: self.re * other.re - self.im * other.im,
      im: self.re * other.im + self.im * other.re,
    }
  }

  pub fn square(self) -> Complex {
    Complex {
      re: self.re * self.re - self.im * self.im,
      im: 2.0 * self.re * self.im,
    }
  }

  pub fn abs(self) -> Complex {
    Complex {
      re: self.re.abs(),
      im: self.im.abs(),
    }
  }

  pub fn polar(power: f64) -> impl Fn(Complex) -> Complex {
    move |z: Complex| {
      let r = z.abs_sq().sqrt();
      let theta = z.im.atan2(z.re);
      let r_pow = r.powf(power);
      let angle = power * theta;
      Complex {
        re: r_pow * angle.cos(),
        im: r_pow * angle.sin(),
      }
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_complex_add() {
    let a = Complex::new(1.0, 2.0);
    let b = Complex::new(3.0, 4.0);
    let sum = a.add(b);
    assert_eq!(sum.re, 4.0);
    assert_eq!(sum.im, 6.0);
  }

  #[test]
  fn test_complex_mul() {
    let a = Complex::new(1.0, 2.0);
    let b = Complex::new(3.0, 4.0);
    let prod = a.mul(b);
    assert_eq!(prod.re, -5.0);
    assert_eq!(prod.im, 10.0);
  }

  #[test]
  fn test_complex_square() {
    let a = Complex::new(2.0, 3.0);
    let squared = a.square();
    assert_eq!(squared.re, -5.0);
    assert_eq!(squared.im, 12.0);
  }

  #[test]
  fn test_complex_abs_sq() {
    let a = Complex::new(3.0, 4.0);
    assert_eq!(a.abs_sq(), 25.0);
  }

  #[test]
  fn test_complex_polar_power_2() {
    let z = Complex::new(1.0, 1.0);
    let polar_fn = Complex::polar(2.0);
    let result = polar_fn(z);
    let expected = z.square();
    assert!((result.re - expected.re).abs() < 1e-10);
    assert!((result.im - expected.im).abs() < 1e-10);
  }
}
