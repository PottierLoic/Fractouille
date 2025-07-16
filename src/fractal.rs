use crate::fractal_colorizer::FractalColorizer;
use crate::fractal_colorizer::generate_image;
use crate::fractal_colorizer::{iterate_point_raw, iterate_point_smooth};
use crate::palettes::{PaletteFn, all_palettes};
use image::{Rgb, RgbImage};
use ratatui::buffer::Buffer;
use ratatui::layout::{Position, Rect};
use ratatui::prelude::{Color, Widget};
use std::thread;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone, PartialEq)]
pub enum Set {
  Mandelbrot,
  Julia,
  BurningShip,
}

#[derive(Debug)]
pub struct Fractal {
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
  pub power: f64,
  pub step: f64,
}

impl Default for Fractal {
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
      power: 2.0,
      step: 0.01,
    }
  }
}

impl Widget for &mut Fractal {
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

impl Fractal {
  fn compute(&mut self, area: Rect) {
    let (w, h) = (area.width as usize, area.height as usize * 2);
    if self.colors.len() == h && self.colors[0].len() == w && !self.need_render {
      return;
    }
    let raw_colors = generate_image(
      &self.set,
      w,
      h,
      self.center_x,
      self.center_y,
      self.scale,
      self.real,
      self.imag,
      |zx, zy, cx, cy| {
        iterate_point_raw(&self.set, zx, zy, cx, cy, self.max_iterations, self.power) as f64
      },
      |iter| {
        <Color as FractalColorizer<Color>>::colorize(
          iter,
          self.max_iterations,
          self.palettes[self.current_palette],
        )
      },
    );
    self.colors = raw_colors;
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
    let power = self.power;

    thread::spawn(move || {
      let (w, h) = (1920, 1080);
      let mut img = RgbImage::new(w, h);

      let colorize = move |iter| Rgb::<u8>::colorize(iter, max_iterations, palette_fn);
      let colors = generate_image(
        &set,
        w as usize,
        h as usize,
        center_x,
        center_y,
        scale,
        real,
        imag,
        |zx, zy, cx, cy| iterate_point_smooth(&set, zx, zy, cx, cy, max_iterations, power),
        colorize,
      );

      for (y, row) in colors.iter().enumerate() {
        for (x, pixel) in row.iter().enumerate() {
          img.put_pixel(x as u32, y as u32, *pixel);
        }
      }

      let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();

      let name = match set {
        Set::Mandelbrot => "mandelbrot",
        Set::Julia => "julia",
        Set::BurningShip => "burningship",
      };
      img
        .save(format!("screenshot_{}_{}.png", name, timestamp))
        .unwrap();
    });
  }
}
