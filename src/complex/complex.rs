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
