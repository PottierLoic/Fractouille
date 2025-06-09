use crate::palettes::{PaletteFn, all_palettes};
use crate::utils::color_to_rgb;
use image::{Rgb, RgbImage};
use ratatui::buffer::Buffer;
use ratatui::layout::{Position, Rect};
use ratatui::prelude::{Color, Widget};
use std::thread;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone)]
pub enum Set {
  Mandelbrot,
  Julia,
}

#[derive(Debug)]
pub struct FractalWidget {
  pub colors: Vec<Vec<Color>>,
  pub center_x: f64,
  pub center_y: f64,
  pub scale: f64,
  pub max_iterations: u32,
  pub need_render: bool,
  pub palettes: Vec<PaletteFn>,
  pub current_palette: usize,
  pub set: Set,
  pub real: f64,
  pub imag: f64,
}

impl Default for FractalWidget {
  fn default() -> Self {
    Self {
      colors: vec![],
      center_x: -0.5,
      center_y: 0.0,
      scale: 1.0,
      max_iterations: 100,
      need_render: true,
      palettes: all_palettes(),
      current_palette: 0,
      set: Set::Mandelbrot,
      real: -0.5251993,
      imag: -0.5251993,
    }
  }
}

impl Widget for &mut FractalWidget {
  fn render(self, area: Rect, buf: &mut Buffer) {
    self.compute(area);

    for (xi, x) in (area.left()..area.right()).enumerate() {
      let xi = (xi + 1) % area.width as usize;
      for (yi, y) in (area.top()..area.bottom()).enumerate() {
        let fg = self.colors[yi * 2][xi];
        let bg = self.colors[yi * 2 + 1][xi];
        buf[Position::new(x, y)].set_char('▀').set_fg(fg).set_bg(bg);
      }
    }
  }
}

impl FractalWidget {
  fn get_viewport(&self, width: usize, height: usize) -> (f64, f64, f64, f64) {
    let aspect = width as f64 / height as f64;
    let vw = 3.5 / self.scale;
    let vh = vw / aspect;
    let left = self.center_x - vw / 2.0;
    let top = self.center_y - vh / 2.0;
    (vw, vh, left, top)
  }

  fn iterate_point(&self, mut zx: f64, mut zy: f64, cx: f64, cy: f64) -> u32 {
    let mut i = 0;
    while zx * zx + zy * zy <= 4.0 && i < self.max_iterations {
      let tmp = zx * zx - zy * zy + cx;
      zy = 2.0 * zx * zy + cy;
      zx = tmp;
      i += 1;
    }
    i
  }

  fn get_color(&self, iterations: u32) -> Color {
    if iterations == self.max_iterations {
      Color::Black
    } else {
      let t = iterations as f64 / self.max_iterations as f64;
      self.palettes[self.current_palette](t)
    }
  }

  fn compute(&mut self, area: Rect) {
    let (w, h) = (area.width as usize, area.height as usize * 2);
    if self.colors.len() == h && self.colors[0].len() == w && !self.need_render {
      return;
    }

    let (vw, vh, left, top) = self.get_viewport(w, h);

    self.colors = (0..h)
      .map(|y| {
        (0..w)
          .map(|x| {
            let (zx, zy, cx, cy) = match self.set {
              Set::Mandelbrot => {
                let cx = left + x as f64 * vw / w as f64;
                let cy = top + y as f64 * vh / h as f64;
                (0.0, 0.0, cx, cy)
              }
              Set::Julia => {
                let zx = left + x as f64 * vw / w as f64;
                let zy = top + y as f64 * vh / h as f64;
                (zx, zy, self.real, self.imag)
              }
            };

            let iterations = self.iterate_point(zx, zy, cx, cy);
            self.get_color(iterations)
          })
          .collect()
      })
      .collect();
  }

  pub fn save_screenshot(&self) {
    let center_x = self.center_x;
    let center_y = self.center_y;
    let scale = self.scale;
    let real = self.real;
    let imag = self.imag;
    let set = self.set.clone();
    let palette_fn = self.palettes[self.current_palette];
    let max_iterations = self.max_iterations;

    thread::spawn(move || {
      let (w, h) = (3840, 2160);
      let mut img = RgbImage::new(w, h);

      let aspect = w as f64 / h as f64;
      let vw = 3.5 / scale;
      let vh = vw / aspect;
      let left = center_x - vw / 2.0;
      let top = center_y - vh / 2.0;

      for y in 0..h {
        for x in 0..w {
          let (zx, zy, cx, cy) = match set {
            Set::Mandelbrot => {
              let cx = left + x as f64 * vw / w as f64;
              let cy = top + y as f64 * vh / h as f64;
              (0.0, 0.0, cx, cy)
            }
            Set::Julia => {
              let zx = left + x as f64 * vw / w as f64;
              let zy = top + y as f64 * vh / h as f64;
              (zx, zy, real, imag)
            }
          };

          let iterations = {
            let mut i = 0;
            let mut zx = zx;
            let mut zy = zy;
            while zx * zx + zy * zy <= 4.0 && i < max_iterations {
              let tmp = zx * zx - zy * zy + cx;
              zy = 2.0 * zx * zy + cy;
              zx = tmp;
              i += 1;
            }

            if i < max_iterations {
              let log_zn = (zx * zx + zy * zy).sqrt().ln().ln();
              i as f64 + 1.0 - log_zn / std::f64::consts::LN_2
            } else {
              i as f64
            }
          };

          let color = if iterations == max_iterations as f64 {
            Color::Black
          } else {
            let t = iterations as f64 / max_iterations as f64;
            palette_fn(t)
          };

          let rgb = color_to_rgb(&color);
          img.put_pixel(x, y, Rgb([rgb.0, rgb.1, rgb.2]));
        }
      }

      let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();

      img.save(format!("screenshot_{}.png", timestamp)).unwrap();
    });
  }
}
