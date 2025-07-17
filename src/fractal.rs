use crate::fractal_colorizer::generate_image;
use image::RgbImage;
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

#[derive(Debug, Clone)]
pub struct Fractal {
  pub colors: Vec<Vec<Color>>,
  pub center_x: f64,
  pub center_y: f64,
  pub scale: f64,
  pub max_iterations: u32,
  pub need_render: bool,
  pub current_palette: usize,
  pub set: Set,
  pub julia_constant: (f64, f64),
  pub power: f64,
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
      current_palette: 0,
      set: Set::Mandelbrot,
      julia_constant: (-0.5251993, -0.5251993),
      power: 2.0,
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
    let raw_colors = generate_image(self, w as u32, h as u32, false);
    self.colors = raw_colors
      .into_iter()
      .map(|row| {
        row
          .into_iter()
          .map(|rgb| Color::Rgb(rgb[0], rgb[1], rgb[2]))
          .collect()
      })
      .collect();
  }

  pub fn save_screenshot(&self) {
    let fractal = self.clone();
    thread::spawn(move || {
      let (width, height) = (1920, 1080);
      let mut img = RgbImage::new(width, height);
      let colors = generate_image(&fractal, width, height, true);

      for (y, row) in colors.iter().enumerate() {
        for (x, pixel) in row.iter().enumerate() {
          img.put_pixel(x as u32, y as u32, *pixel);
        }
      }

      let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();

      let name = match fractal.set {
        Set::Mandelbrot => "mandelbrot",
        Set::Julia => "julia",
        Set::BurningShip => "burningship",
      };
      img
        .save(format!(
          "{}_{}_x{}_y{}_z{}_p{}.png",
          name, timestamp, fractal.center_x, fractal.center_y, fractal.scale, fractal.power
        ))
        .unwrap();
    });
  }
}
