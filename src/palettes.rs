#[derive(Debug, Clone)]
pub enum InterpolationMode {
  Linear,
  Cosine,
}

#[derive(Debug, Clone)]
pub struct Palette {
  pub stops: Vec<(u8, u8, u8)>,
  pub interpolation: InterpolationMode,
  pub cycle_speed: f64,
}

impl Palette {
  pub fn eval(&self, t: f64) -> (u8, u8, u8) {
    match self.interpolation {
      InterpolationMode::Linear => self.eval_linear(t),
      InterpolationMode::Cosine => self.eval_cosine(t),
    }
  }

  pub fn eval_linear(&self, t: f64) -> (u8, u8, u8) {
    let n = self.stops.len();

    // no stops should never happen
    if n == 0 {
      return (0, 0, 0);
    }

    // only one color
    if n == 1 {
      return self.stops[0];
    }

    let scaled = t.clamp(0.0, 1.0) * (n - 1) as f64;
    let i = scaled.floor() as usize;
    let frac = scaled - i as f64;

    let (r1, g1, b1) = self.stops[i];
    let (r2, g2, b2) = self.stops[(i + 1).min(n - 1)];

    let r = r1 as f64 + frac * (r2 as f64 - r1 as f64);
    let g = g1 as f64 + frac * (g2 as f64 - g1 as f64);
    let b = b1 as f64 + frac * (b2 as f64 - b1 as f64);

    (r as u8, g as u8, b as u8)
  }

  pub fn eval_cosine(&self, t: f64) -> (u8, u8, u8) {
    todo!();
  }
}

pub fn default_palettes() -> Vec<Palette> {
  vec![
    // 1. Default polynomial-like approximation
    Palette {
      stops: vec![(30, 0, 50), (120, 10, 120), (200, 80, 20), (250, 200, 40)],
      interpolation: InterpolationMode::Linear,
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
