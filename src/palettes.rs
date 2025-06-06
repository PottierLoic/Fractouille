use ratatui::prelude::Color;

pub type PaletteFn = fn(t: f64) -> Color;

pub fn default_palette(t: f64) -> Color {
  let (r, g, b) = (
    (9.0 * (1.0 - t) * t * t * t * 255.0) as u8,
    (15.0 * (1.0 - t).powi(2) * t * t * 255.0) as u8,
    (8.5 * (1.0 - t).powi(3) * t * 255.0) as u8,
  );
  Color::Rgb(r, g, b)
}

pub fn fire_palette(t: f64) -> Color {
  let r = (255.0 * t) as u8;
  let g = (255.0 * t.powf(0.5) * (1.0 - t)) as u8;
  let b = (64.0 * (1.0 - t)) as u8;
  Color::Rgb(r, g, b)
}

pub fn rainbow_palette(t: f64) -> Color {
  let r = (127.5 * (1.0 + (6.0 * t).sin())) as u8;
  let g = (127.5 * (1.0 + (6.0 * t + 2.0).sin())) as u8;
  let b = (127.5 * (1.0 + (6.0 * t + 4.0).sin())) as u8;
  Color::Rgb(r, g, b)
}

pub fn ocean_palette(t: f64) -> Color {
  let r = (20.0 * (1.0 - t)) as u8;
  let g = (80.0 + 120.0 * t) as u8;
  let b = (200.0 + 55.0 * t) as u8;
  Color::Rgb(r, g, b)
}

pub fn grayscale_palette(t: f64) -> Color {
  let shade = (255.0 * t) as u8;
  Color::Rgb(shade, shade, shade)
}

pub fn electric_palette(t: f64) -> Color {
  let r = (100.0 * (1.0 - t)) as u8;
  let g = (200.0 * t) as u8;
  let b = (255.0 * (t * 1.2).min(1.0)) as u8;
  Color::Rgb(r, g, b)
}

pub fn all_palettes() -> Vec<fn(f64) -> Color> {
  vec![
    default_palette,
    fire_palette,
    rainbow_palette,
    ocean_palette,
    grayscale_palette,
    electric_palette,
  ]
}
