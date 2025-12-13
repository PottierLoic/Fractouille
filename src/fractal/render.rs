use std::ptr;
use std::sync::{Arc, Mutex};
use std::sync::atomic::AtomicPtr;
use crate::complex::Complex;
use crate::fractal::constants::{LOG2, SMOOTH_OFFSET};
use crate::fractal::iter::{
  iterate_burningship, iterate_julia, iterate_mandelbrot, iterate_phoenix,
};
use crate::fractal::{Fractal, Set};
use image::Rgb;
use rayon::scope;

const TOP_TILE: u32 = 512;
const MIN_TILE: u32 = 8;

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

    let mut out = vec![Rgb([0, 0, 0]); size];

    let mut initial_tiles = Vec::new();
    for y in (0..height).step_by(TOP_TILE as usize) {
      for x in (0..width).step_by(TOP_TILE as usize) {
        initial_tiles.push(Tile {
          x0: x,
          y0: y,
          w: TOP_TILE.min(width - x),
          h: TOP_TILE.min(height - y),
        });
      }
    }

    let work_pool = Arc::new(Mutex::new(initial_tiles));
    let out_atomic_ptr = Arc::new(AtomicPtr::new(out.as_mut_ptr()));

    scope(|s| {
      for _ in 0..rayon::current_num_threads() {
        let pool_clone = work_pool.clone();
        let out_atomic_ptr_clone = out_atomic_ptr.clone();

        s.spawn(move |_| unsafe {
          let out_ptr = out_atomic_ptr_clone.load(std::sync::atomic::Ordering::Relaxed);

          work_loop_unsafe(
            pool_clone,
            out_ptr,
            width,
            height,
            left,
            top,
            vw,
            vh,
            smooth,
            self,
          );
        });
      }
    });

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

unsafe fn work_loop_unsafe(
  work_pool: Arc<Mutex<Vec<Tile>>>,
  out_ptr: *mut Rgb<u8>,
  width: u32,
  height: u32,
  left: f64,
  top: f64,
  vw: f64,
  vh: f64,
  smooth: bool,
  fractal: &Fractal,
) {
  loop {
    let current_tile = {
      let mut pool = match work_pool.lock() {
        Ok(p) => p,
        Err(_) => break,
      };
      match pool.pop() {
        Some(tile) => tile,
        None => break,
      }
    };

    let mut stack = vec![current_tile];
    while let Some(current_sub_tile) = stack.pop() {
      let mut check_and_compute = |x: u32, y: u32| -> Rgb<u8> {
        let val = compute(x, y, width, height, left, top, vw, vh, smooth, fractal);

        let index = idx(width, x, y);
        ptr::write(out_ptr.add(index), val);
        val
      };

      let x1 = current_sub_tile.x0 + current_sub_tile.w - 1;
      let y1 = current_sub_tile.y0 + current_sub_tile.h - 1;

      if current_sub_tile.w <= MIN_TILE || current_sub_tile.h <= MIN_TILE {
        for x in current_sub_tile.x0..=x1 {
          for y in current_sub_tile.y0..=y1 {
            check_and_compute(x, y);
          }
        }
        continue;
      }

      let mut is_interior = true;
      let mut edge_results = Vec::new();

      for x in current_sub_tile.x0..=x1 {
        let val_t = check_and_compute(x, current_sub_tile.y0);
        edge_results.push(val_t);
        if val_t != Rgb([0, 0, 0]) { is_interior = false; }
        if y1 != current_sub_tile.y0 {
          let val_b = check_and_compute(x, y1);
          edge_results.push(val_b);
          if val_b != Rgb([0, 0, 0]) { is_interior = false; }
        }
      }

      for y in (current_sub_tile.y0 + 1)..y1 {
        let val_l = check_and_compute(current_sub_tile.x0, y);
        edge_results.push(val_l);
        if val_l != Rgb([0, 0, 0]) { is_interior = false; }
        if x1 != current_sub_tile.x0 {
          let val_r = check_and_compute(x1, y);
          edge_results.push(val_r);
          if val_r != Rgb([0, 0, 0]) { is_interior = false; }
        }
      }

      if is_interior {
        for x in (current_sub_tile.x0 + 1)..(current_sub_tile.x0 + current_sub_tile.w - 1) {
          for y in (current_sub_tile.y0 + 1)..(current_sub_tile.y0 + current_sub_tile.h - 1) {
            let index = idx(width, x, y);
            ptr::write(out_ptr.add(index), Rgb([255, 255, 255]));
          }
        }
      } else {
        let hw = current_sub_tile.w / 2;
        let hh = current_sub_tile.h / 2;
        let w0 = hw;
        let h0 = hh;
        let w1 = current_sub_tile.w - hw;
        let h1 = current_sub_tile.h - hh;

        let children = vec![
          Tile { x0: current_sub_tile.x0, y0: current_sub_tile.y0, w: w0, h: h0, },
          Tile { x0: current_sub_tile.x0 + w0, y0: current_sub_tile.y0, w: w1, h: h0, },
          Tile { x0: current_sub_tile.x0, y0: current_sub_tile.y0 + h0, w: w0, h: h1, },
          Tile { x0: current_sub_tile.x0 + w0, y0: current_sub_tile.y0 + h0, w: w1, h: h1, },
        ];

        let mut pool = work_pool.lock().unwrap();
        pool.extend(children);
      }

    }
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
        (255, z0)
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
