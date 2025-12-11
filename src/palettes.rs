use crate::palette_helpers::{hsv_to_rgb, rgb_to_hsv};

#[derive(Debug, Clone)]
pub enum InterpolationMode {
  Linear,
  Cosine,
  HSV,
  HSVCyclic
}

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
      InterpolationMode::Linear => self.eval_linear(t),
      InterpolationMode::Cosine => self.eval_cosine(t),
      InterpolationMode::HSV => self.eval_hsv(t),
      InterpolationMode::HSVCyclic => self.eval_hsv_cyclic(t),
    }
  }

  pub fn resolve_segment(&self, t: f64) -> (usize, usize, f64) {
    let n = self.stops.len();

    if n == 0 {
      return (0, 0, 0.0)
    }
    if n == 1 {
      return (0, 0, 1.0)
    }

    let scaled = t.clamp(0.0, 1.0) * (n - 1) as f64;
    let i1 = scaled.floor() as usize % n;
    let i2 = (i1 + 1) % n;
    let local_t = scaled - scaled.floor();

    (i1, i2, local_t)
  }

  pub fn eval_linear(&self, t: f64) -> (u8, u8, u8) {
    let (i1, i2, local_t) = self.resolve_segment(t);

    let (r1, g1, b1) = self.stops[i1];
    let (r2, g2, b2) = self.stops[i2];

    let r = r1 as f64 + local_t * (r2 as f64 - r1 as f64);
    let g = g1 as f64 + local_t * (g2 as f64 - g1 as f64);
    let b = b1 as f64 + local_t * (b2 as f64 - b1 as f64);

    (r as u8, g as u8, b as u8)
  }

  pub fn eval_cosine(&self, t: f64) -> (u8, u8, u8) {
    let (i1, i2, local_t) = self.resolve_segment(t);

    let (r1, g1, b1) = self.stops[i1];
    let (r2, g2, b2) = self.stops[i2];

    let mu = (1.0 - (std::f64::consts::PI * local_t).cos()) / 2.0;

    let r = r1 as f64 * (1.0 - mu) + r2 as f64 * mu;
    let g = g1 as f64 * (1.0 - mu) + g2 as f64 * mu;
    let b = b1 as f64 * (1.0 - mu) + b2 as f64 * mu;

    (r as u8, g as u8, b as u8)
  }

  pub fn eval_hsv(&self, t: f64) -> (u8, u8, u8) {
    let (i1, i2, local_t) = self.resolve_segment(t);

    let (r1, g1, b1) = self.stops[i1];
    let (r2, g2, b2) = self.stops[i2];

    let (h1, s1, v1) = rgb_to_hsv(r1, g1, b1);
    let (h2, s2, v2) = rgb_to_hsv(r2, g2, b2);

    let h = h1 + local_t * (h2 - h1);
    let s = s1 + local_t * (s2 - s1);
    let v = v1 + local_t * (v2 - v1);

    hsv_to_rgb(h, s, v)
  }

  pub fn eval_hsv_cyclic(&self, t: f64) -> (u8, u8, u8) {
    let (i1, i2, local_t) = self.resolve_segment(t);

    let (r1, g1, b1) = self.stops[i1];
    let (r2, g2, b2) = self.stops[i2];

    let (h1, s1, v1) = rgb_to_hsv(r1, g1, b1);
    let (h2, s2, v2) = rgb_to_hsv(r2, g2, b2);

    let mut dh = h2 - h1;
    if dh > 180.0 {
      dh -= 360.0;
    } else if dh < -180.0 {
      dh += 360.0;
    }

    let h = h1 + dh * local_t.rem_euclid(360.0);
    let s = s1 + local_t * (s2 - s1);
    let v = v1 + local_t * (v2 - v1);

    hsv_to_rgb(h, s, v)
  }
}

pub fn default_palettes() -> Vec<Palette> {
  vec![
    // 1. Default polynomial-like approximation
    Palette {
      stops: vec![(30, 0, 50), (120, 10, 120), (200, 80, 20), (250, 200, 40)],
      interpolation: InterpolationMode::Cosine,
      cycle_speed: 100.0,
    },
    // 2. Fire palette
    Palette {
      stops: vec![
        (0, 0, 0),
        (80, 0, 0),
        (200, 30, 0),
        (255, 140, 0),
        (255, 255, 100),
      ],
      interpolation: InterpolationMode::Linear,
      cycle_speed: 100.0,
    },
    // 3. Ocean palette
    Palette {
      stops: vec![
        (0, 0, 30),
        (0, 40, 120),
        (0, 110, 180),
        (80, 200, 255),
        (200, 255, 255),
      ],
      interpolation: InterpolationMode::Linear,
      cycle_speed: 100.0,
    },
    // 4. Ice palette
    Palette {
      stops: vec![
        (240, 240, 255),
        (180, 220, 255),
        (120, 170, 255),
        (40, 100, 200),
        (10, 30, 120),
      ],
      interpolation: InterpolationMode::Linear,
      cycle_speed: 90.0,
    },
    // 5. Neon palette
    Palette {
      stops: vec![
        (255, 0, 150),
        (80, 0, 255),
        (0, 120, 255),
        (0, 255, 180),
        (180, 255, 0),
      ],
      interpolation: InterpolationMode::Linear,
      cycle_speed: 120.0,
    },
    // 6. Earth palette
    Palette {
      stops: vec![
        (40, 20, 0),
        (120, 70, 20),
        (180, 140, 60),
        (40, 160, 60),
        (120, 200, 180),
      ],
      interpolation: InterpolationMode::Linear,
      cycle_speed: 80.0,
    },
  ]
}
