use crate::palette::palette::Palette;
use crate::palette::utils::{hsv_to_rgb, rgb_to_hsv};

#[derive(Debug, Clone)]
pub enum InterpolationMode {
  Linear,
  Cosine,
  Hsv,
  HsvCyclic,
}

pub fn eval_linear(p: &Palette, t: f64) -> (u8, u8, u8) {
  let (i1, i2, local_t) = p.resolve_segment(t);

  let (r1, g1, b1) = p.stops[i1];
  let (r2, g2, b2) = p.stops[i2];

  let r = r1 as f64 + local_t * (r2 as f64 - r1 as f64);
  let g = g1 as f64 + local_t * (g2 as f64 - g1 as f64);
  let b = b1 as f64 + local_t * (b2 as f64 - b1 as f64);

  (r as u8, g as u8, b as u8)
}

pub fn eval_cosine(p: &Palette, t: f64) -> (u8, u8, u8) {
  let (i1, i2, local_t) = p.resolve_segment(t);

  let (r1, g1, b1) = p.stops[i1];
  let (r2, g2, b2) = p.stops[i2];

  let mu = (1.0 - (std::f64::consts::PI * local_t).cos()) / 2.0;

  let r = r1 as f64 * (1.0 - mu) + r2 as f64 * mu;
  let g = g1 as f64 * (1.0 - mu) + g2 as f64 * mu;
  let b = b1 as f64 * (1.0 - mu) + b2 as f64 * mu;

  (r as u8, g as u8, b as u8)
}

pub fn eval_hsv(p: &Palette, t: f64) -> (u8, u8, u8) {
  let (i1, i2, local_t) = p.resolve_segment(t);

  let (r1, g1, b1) = p.stops[i1];
  let (r2, g2, b2) = p.stops[i2];

  let (h1, s1, v1) = rgb_to_hsv(r1, g1, b1);
  let (h2, s2, v2) = rgb_to_hsv(r2, g2, b2);

  let h = h1 + local_t * (h2 - h1);
  let s = s1 + local_t * (s2 - s1);
  let v = v1 + local_t * (v2 - v1);

  hsv_to_rgb(h, s, v)
}

pub fn eval_hsv_cyclic(p: &Palette, t: f64) -> (u8, u8, u8) {
  let (i1, i2, local_t) = p.resolve_segment(t);

  let (r1, g1, b1) = p.stops[i1];
  let (r2, g2, b2) = p.stops[i2];

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
