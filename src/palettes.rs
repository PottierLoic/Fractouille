pub type PaletteFn = fn(t: f64) -> (u8, u8, u8);

pub fn default_palette(t: f64) -> (u8, u8, u8) {
  (
    (9.0 * (1.0 - t) * t * t * t * 255.0) as u8,
    (15.0 * (1.0 - t).powi(2) * t * t * 255.0) as u8,
    (8.5 * (1.0 - t).powi(3) * t * 255.0) as u8,
  )
}

pub fn fire_palette(t: f64) -> (u8, u8, u8) {
  (
    (255.0 * t) as u8,
    (255.0 * t.powf(0.5) * (1.0 - t)) as u8,
    (64.0 * (1.0 - t)) as u8,
  )
}

pub fn rainbow_palette(t: f64) -> (u8, u8, u8) {
  (
    (127.5 * (1.0 + (6.0 * t).sin())) as u8,
    (127.5 * (1.0 + (6.0 * t + 2.0).sin())) as u8,
    (127.5 * (1.0 + (6.0 * t + 4.0).sin())) as u8,
  )
}

pub fn ocean_palette(t: f64) -> (u8, u8, u8) {
  (
    (20.0 * (1.0 - t)) as u8,
    (80.0 + 120.0 * t) as u8,
    (200.0 + 55.0 * t) as u8,
  )
}

pub fn grayscale_palette(t: f64) -> (u8, u8, u8) {
  let shade = (255.0 * t) as u8;
  (shade, shade, shade)
}

pub fn electric_palette(t: f64) -> (u8, u8, u8) {
  (
    (100.0 * (1.0 - t)) as u8,
    (200.0 * t) as u8,
    (255.0 * (t * 1.2).min(1.0)) as u8,
  )
}

pub const PALETTES: &[PaletteFn] = &[
  default_palette,
  fire_palette,
  rainbow_palette,
  ocean_palette,
  grayscale_palette,
  electric_palette,
];
