use crate::fractal::{Fractal, Set};
use crate::palettes::{PALETTES, PaletteFn};
use image::Rgb;
use rayon::prelude::*;

fn colorize(iter: f64, max_iter: u32, palette: PaletteFn) -> Rgb<u8> {
  let (r, g, b) = if iter >= max_iter as f64 {
    (0, 0, 0)
  } else {
    palette(iter / max_iter as f64)
  };
  Rgb([r, g, b])
}

fn iterate_point(
  set: &Set,
  mut zx: f64,
  mut zy: f64,
  cx: f64,
  cy: f64,
  max_iterations: u32,
  power: f64,
) -> (u32, f64, f64) {
  let mut i = 0;
  match set {
    Set::BurningShip => {
      while zx * zx + zy * zy <= 4.0 && i < max_iterations {
        let abs_zx = zx.abs();
        let abs_zy = zy.abs();
        let tmp = abs_zx * abs_zx - abs_zy * abs_zy + cx;
        zy = 2.0 * abs_zx * abs_zy + cy;
        zx = tmp;
        i += 1;
      }
    }
    _ => {
      if power == 2.0 {
        while zx * zx + zy * zy <= 4.0 && i < max_iterations {
          let tmp = zx * zx - zy * zy + cx;
          zy = 2.0 * zx * zy + cy;
          zx = tmp;
          i += 1;
        }
      } else {
        while zx * zx + zy * zy <= 4.0 && i < max_iterations {
          let r = (zx * zx + zy * zy).sqrt();
          let theta = zy.atan2(zx);
          let r_pow = r.powf(power);
          let angle = power * theta;

          zx = r_pow * angle.cos() + cx;
          zy = r_pow * angle.sin() + cy;
          i += 1;
        }
      }
    }
  }

  (i, zx, zy)
}

fn iterate_point_raw(
  set: &Set,
  zx: f64,
  zy: f64,
  cx: f64,
  cy: f64,
  max_iterations: u32,
  power: f64,
) -> u32 {
  let (i, _, _) = iterate_point(set, zx, zy, cx, cy, max_iterations, power);
  i
}

fn iterate_point_smooth(
  set: &Set,
  zx: f64,
  zy: f64,
  cx: f64,
  cy: f64,
  max_iterations: u32,
  power: f64,
) -> f64 {
  let (i, zx, zy) = iterate_point(set, zx, zy, cx, cy, max_iterations, power);
  if i < max_iterations {
    let log_zn = (zx * zx + zy * zy).sqrt().ln().ln();
    i as f64 + 1.0 - log_zn / std::f64::consts::LN_2
  } else {
    i as f64
  }
}

pub fn generate_image(
  fractal: &Fractal,
  width: u32,
  height: u32,
  smooth: bool,
) -> Vec<Vec<Rgb<u8>>> {
  let aspect = width as f64 / height as f64;
  let vw = 3.5 / fractal.scale;
  let vh = vw / aspect;
  let left = fractal.center_x - vw / 2.0;
  let top = fractal.center_y - vh / 2.0;

  (0..height)
    .into_par_iter()
    .map(|y| {
      (0..width)
        .map(|x| {
          let (zx, zy, cx, cy) = match fractal.set {
            Set::Mandelbrot | Set::BurningShip => {
              let cx = left + x as f64 * vw / width as f64;
              let cy = top + y as f64 * vh / height as f64;
              (0.0, 0.0, cx, cy)
            }
            Set::Julia => {
              let zx = left + x as f64 * vw / width as f64;
              let zy = top + y as f64 * vh / height as f64;
              (zx, zy, fractal.julia_constant.0, fractal.julia_constant.1)
            }
          };

          let iter = if smooth {
            iterate_point_smooth(
              &fractal.set,
              zx,
              zy,
              cx,
              cy,
              fractal.max_iterations,
              fractal.power,
            )
          } else {
            iterate_point_raw(
              &fractal.set,
              zx,
              zy,
              cx,
              cy,
              fractal.max_iterations,
              fractal.power,
            ) as f64
          };

          colorize(
            iter,
            fractal.max_iterations,
            PALETTES[fractal.current_palette],
          )
        })
        .collect()
    })
    .collect()
}
