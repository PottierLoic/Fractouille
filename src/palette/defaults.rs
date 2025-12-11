use crate::palette::interpolation::InterpolationMode;
use crate::palette::palette::Palette;

pub fn default_palettes() -> Vec<Palette> {
  vec![
    // 1. Default
    Palette {
      stops: vec![(30, 0, 50), (120, 10, 120), (200, 80, 20), (250, 200, 40)],
      interpolation: InterpolationMode::Cosine,
      cycle_speed: 20.0,
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
      cycle_speed: 20.0,
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
      cycle_speed: 20.0,
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
      cycle_speed: 20.0,
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
      cycle_speed: 20.0,
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
      cycle_speed: 20.0,
    },
    // 7. Rainbow
    Palette {
      stops: vec![
        (255, 0, 0),
        (255, 127, 0),
        (255, 255, 0),
        (0, 255, 0),
        (0, 0, 255),
        (139, 0, 255),
        (255, 0, 0),
      ],
      interpolation: InterpolationMode::HsvCyclic,
      cycle_speed: 50.0,
    },
  ]
}
