use crate::palette::interpolation::*;

#[derive(Debug, Clone)]
pub struct Palette {
  pub stops: Vec<(u8, u8, u8)>,
  pub interpolation: InterpolationMode,
  pub cycle_speed: f64,
}

impl Palette {
  pub fn new(stops: Vec<(u8, u8, u8)>, interpolation: InterpolationMode, cycle_speed: f64) -> Self {
    Palette {
      stops,
      interpolation,
      cycle_speed,
    }
  }

  pub fn eval(&self, t: f64) -> (u8, u8, u8) {
    match self.interpolation {
      InterpolationMode::Linear => eval_linear(self, t),
      InterpolationMode::Cosine => eval_cosine(self, t),
      InterpolationMode::Hsv => eval_hsv(self, t),
      InterpolationMode::HsvCyclic => eval_hsv_cyclic(self, t),
      InterpolationMode::None => eval_none(self, t),
    }
  }

  pub fn resolve_segment(&self, t: f64) -> (usize, usize, f64) {
    let n = self.stops.len();

    if n == 0 {
      return (0, 0, 0.0);
    }
    if n == 1 {
      return (0, 0, 1.0);
    }

    let scaled = t.rem_euclid(n as f64);
    let i1 = scaled.floor() as usize % n;
    let i2 = (i1 + 1) % n;
    let local_t = scaled - scaled.floor();

    (i1, i2, local_t)
  }
}
