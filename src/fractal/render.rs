use crate::complex::Complex;
use crate::fractal::constants::{LOG2, SMOOTH_OFFSET};
use crate::fractal::iter::{
  iterate_burningship, iterate_julia, iterate_mandelbrot, iterate_phoenix,
};
use crate::fractal::{Fractal, Set};
use image::Rgb;
use rayon::iter::IntoParallelIterator;
use rayon::iter::ParallelIterator;

const TOP_TILE: u32 = 64;

#[derive(Clone, Copy)]
struct Tile {
  x0: u32,
  y0: u32,
  w: u32,
  h: u32,
}

impl Fractal {
  pub fn render_frame(&self, width: u32, height: u32, smooth: bool) -> Vec<Vec<Rgb<u8>>> {
    let aspect = width as f64 / height as f64;
    let vw = 3.5 / self.scale;
    let vh = vw / aspect;
    let left = self.z.re - vw / 2.0;
    let top = self.z.im - vh / 2.0;

    let size = width as usize * height as usize;

    let mut tiles = Vec::new();
    for y in (0..height).step_by(TOP_TILE as usize) {
      for x in (0..width).step_by(TOP_TILE as usize) {
        tiles.push(Tile {
          x0: x,
          y0: y,
          w: TOP_TILE.min(width - x),
          h: TOP_TILE.min(height - y),
        });
      }
    }

    let results: Vec<(Tile, Vec<Rgb<u8>>)> = tiles
        .into_par_iter()
        .map(|tile| {
          let tile_data = process_tile(tile, width, height, left, top, vw, vh, smooth, self);
          (tile, tile_data)
        })
        .collect();

    let mut out = vec![Rgb([0, 0, 0]); size];

    for (tile, data) in results {
      let mut data_iter = data.into_iter();
      for y in tile.y0..(tile.y0 + tile.h) {
        for x in tile.x0..(tile.x0 + tile.w) {
          out[idx(width, x, y)] = data_iter.next().unwrap();
        }
      }
    }

    out.chunks(width as usize).map(|row| row.to_vec()).collect()
  }

  pub fn colorize(&self, iter: f64) -> Rgb<u8> {
    if iter >= self.max_iterations as f64 {
      return Rgb([0, 0, 0]);
    }

    let palette = &self.palette[self.current_palette];
    let (r, g, b) = palette.eval(iter / palette.cycle_speed);
    Rgb([r, g, b])
  }
}

#[inline(always)]
fn in_bulb(c: Complex) -> bool {
  let x = c.re - 0.25;
  let y = c.im;
  let q = x * x + y * y;
  if q * (q + x) <= 0.25 * y * y {
    return true;
  }

  let dx = c.re + 1.0;
  if dx * dx + c.im * c.im <= 0.0625 {
    return true;
  }

  false
}

#[inline(always)]
fn compute(
  x: u32,
  y: u32,
  width: u32,
  height: u32,
  left: f64,
  top: f64,
  vw: f64,
  vh: f64,
  smooth: bool,
  fractal: &Fractal,
) -> Rgb<u8> {
  let cx = left + x as f64 * vw / width as f64;
  let cy = top + y as f64 * vh / height as f64;
  let c0 = Complex::new(cx, cy);
  let z0 = Complex::new(0.0, 0.0);

  let (iter, final_z) = match fractal.set {
    Set::Mandelbrot => {
      if fractal.power == 2.0 && in_bulb(c0) {
        (fractal.max_iterations, z0)
      } else {
        iterate_mandelbrot(z0, c0, fractal.max_iterations, fractal.power)
      }
    }
    Set::Julia => iterate_julia(z0, c0, fractal.max_iterations, fractal.power),
    Set::BurningShip => iterate_burningship(z0, c0, fractal.max_iterations),
    Set::Phoenix => iterate_phoenix(
      z0,
      c0,
      fractal.phoenix_p,
      fractal.max_iterations,
      fractal.power,
    ),
  };

  let value = if smooth && iter < fractal.max_iterations {
    let log_zn = final_z.abs_sq().sqrt().ln().ln();
    iter as f64 + SMOOTH_OFFSET - log_zn / LOG2
  } else {
    iter as f64
  };

  let color = fractal.colorize(value);
  color
}

#[inline(always)]
fn idx(width: u32, x: u32, y: u32) -> usize {
  (y as usize) * (width as usize) + (x as usize)
}

#[inline(always)]
fn local_idx(x: u32, y: u32, tile_w: u32, tile_x0: u32, tile_y0: u32) -> usize {
  ((y - tile_y0) * tile_w + (x - tile_x0)) as usize
}

fn process_tile(
  tile: Tile,
  width: u32,
  height: u32,
  left: f64,
  top: f64,
  vw: f64,
  vh: f64,
  smooth: bool,
  fractal: &Fractal,
) -> Vec<Rgb<u8>> {
  let mut tile_pixels = vec![Rgb([0, 0, 0]); (tile.w * tile.h) as usize];
  let mut calculated_local = vec![false; (tile.w * tile.h) as usize];

  let lid = |x: u32, y: u32| -> usize { local_idx(x, y, tile.w, tile.x0, tile.y0) };

  let mut check_and_compute = |x: u32, y: u32| -> Rgb<u8> {
    let index = lid(x, y);

    if calculated_local[index] {
      return tile_pixels[index];
    }

    let val = compute(x, y, width, height, left, top, vw, vh, smooth, fractal);
    tile_pixels[index] = val;
    calculated_local[index] = true;
    val
  };

  let x1 = tile.x0 + tile.w - 1;
  let y1 = tile.y0 + tile.h - 1;

  let mut is_interior = true;
  for x in tile.x0..=x1 {
    let val = check_and_compute(x, tile.y0);
    if val != Rgb([0, 0, 0]) {
      is_interior = false;
    }
    if y1 != tile.y0 {
      let val = check_and_compute(x, y1);
      if val != Rgb([0, 0, 0]) {
        is_interior = false;
      }
    }
  }

  for y in (tile.y0 + 1)..y1 {
    let val = check_and_compute(tile.x0, y);
    if val != Rgb([0, 0, 0]) {
      is_interior = false;
    }
    if x1 != tile.x0 {
      let val = check_and_compute(x1, y);
      if val != Rgb([0, 0, 0]) {
        is_interior = false;
      }
    }
  }

  if is_interior {
    for x in (tile.x0 + 1)..(tile.x0 + tile.w - 1) {
      for y in (tile.y0 + 1)..(tile.y0 + tile.h - 1) {
        let index = lid(x, y);
        tile_pixels[index] = Rgb([255, 255, 255]);
        calculated_local[index] = true;
      }
    }
  } else {
    for x in tile.x0..=x1 {
      for y in tile.y0..=y1 {
        check_and_compute(x, y);
      }
    }
  }

  tile_pixels
}
