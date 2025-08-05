use crate::complex::Complex;
use crate::fractal_maths::generate_image;
use color_eyre::eyre::Result;
use image::RgbImage;
use ratatui::buffer::Buffer;
use ratatui::layout::{Position, Rect};
use ratatui::prelude::{Color, Widget};
use std::fs;
use std::path::PathBuf;
use std::thread;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone, PartialEq)]
pub enum Set {
  Mandelbrot,
  Julia,
  BurningShip,
  Phoenix,
}

#[derive(Debug, Clone)]
pub struct Fractal {
  pub colors: Vec<Vec<Color>>,
  pub z: Complex,
  pub scale: f64,
  pub max_iterations: u32,
  pub need_render: bool,
  pub current_palette: usize,
  pub set: Set,
  pub julia_c: Complex,
  pub phoenix_c: Complex,
  pub phoenix_p: Complex,
  pub power: f64,
  pub color_cycle: f64,
}

impl Default for Fractal {
  fn default() -> Self {
    Self {
      colors: vec![],
      z: Complex::new(-0.5, 0.0),
      scale: 1.0,
      max_iterations: 100,
      need_render: true,
      current_palette: 0,
      set: Set::Mandelbrot,
      julia_c: Complex::new(-0.5251993, -0.5251993),
      phoenix_c: Complex::new(0.0, 0.0),
      phoenix_p: Complex::new(-0.5, 0.0),
      power: 2.0,
      color_cycle: 100.0,
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

  pub fn save_screenshot(
    &self,
    width: Option<u32>,
    height: Option<u32>,
  ) -> Result<PathBuf, String> {
    let fractal = self.clone();
    let width = width.unwrap_or(1920);
    let height = height.unwrap_or(1080);

    let base_dir = dirs::picture_dir()
      .or_else(dirs::home_dir)
      .ok_or_else(|| "Could not determine user directory".to_string())?;

    let screenshots_dir = base_dir.join("fractouille_screenshots");
    fs::create_dir_all(&screenshots_dir).map_err(|e| e.to_string())?;

    let thread_screenshots_dir = screenshots_dir.clone();

    thread::spawn(move || -> Result<PathBuf> {
      let mut img = RgbImage::new(width, height);
      let colors = generate_image(&fractal, width, height, true);

      for (y, row) in colors.iter().enumerate() {
        for (x, pixel) in row.iter().enumerate() {
          img.put_pixel(x as u32, y as u32, *pixel);
        }
      }

      let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();

      let name = match fractal.set {
        Set::Mandelbrot => "mandelbrot",
        Set::Julia => "julia",
        Set::BurningShip => "burningship",
        Set::Phoenix => "phoenix",
      };

      let filename = format!(
        "{}_{}_x{}_y{}_z{}_p{}.png",
        name, timestamp, fractal.z.re, fractal.z.im, fractal.scale, fractal.power
      );

      let file_path = thread_screenshots_dir.join(&filename);
      img
        .save(&file_path)
        .map_err(|e| color_eyre::eyre::eyre!("Failed to save screenshot: {}", e))?;

      Ok(file_path)
    });

    Ok(screenshots_dir)
  }

  pub fn record_zoom(
    &self,
    width: u32,
    height: u32,
    start_scale: f64,
    end_scale: f64,
    zoom_speed: f64,
  ) -> Result<PathBuf, String> {
    let fractal = self.clone();
    let base_dir = dirs::picture_dir()
      .or_else(dirs::home_dir)
      .ok_or_else(|| "Could not determine user directory".to_string())?;
    let timestamp = SystemTime::now()
      .duration_since(UNIX_EPOCH)
      .unwrap_or_default()
      .as_secs();
    let default_dir = format!("fractouille_records/zoom_{}", timestamp);
    let output_path = base_dir.join(default_dir);
    fs::create_dir_all(&output_path).map_err(|e| e.to_string())?;

    let thread_output_path = output_path.clone();

    thread::spawn(move || -> Result<PathBuf> {
      let fps = 30.0;
      let total_frames = (end_scale.ln() - start_scale.ln()).abs() / zoom_speed * fps;
      let total_frames = total_frames.ceil() as u32;
      if total_frames == 0 {
        return Ok(thread_output_path);
      }

      let mut thread_fractal = fractal;
      for frame in 0..total_frames {
        let t = frame as f64 / total_frames as f64;
        let scale = start_scale * (end_scale / start_scale).powf(t);
        thread_fractal.scale = scale;

        let colors = generate_image(&thread_fractal, width, height, true);
        let mut img = RgbImage::new(width, height);
        for (y, row) in colors.iter().enumerate() {
          for (x, pixel) in row.iter().enumerate() {
            img.put_pixel(x as u32, y as u32, *pixel);
          }
        }

        let frame_path = thread_output_path.join(format!("frame_{:04}.png", frame));
        img
          .save(&frame_path)
          .map_err(|e| color_eyre::eyre::eyre!("Failed to save frame: {}", e))?;
      }

      Ok(thread_output_path)
    });

    Ok(output_path)
  }
}
