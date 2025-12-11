use crate::ProgressEvent;
use crate::complex::Complex;
use crate::fractal_iterator::{
  BurningShipIterator, FractalIterator, MandelbrotIterator, PhoenixIterator,
};
use crate::palettes::{Palette, default_palettes};
use color_eyre::eyre::Result;
use image::{Rgb, RgbImage};
use ratatui::buffer::Buffer;
use ratatui::layout::{Position, Rect};
use ratatui::prelude::{Color, Widget};
use rayon::iter::IntoParallelIterator;
use rayon::iter::ParallelIterator;
use std::fs;
use std::path::PathBuf;
use std::process::{Command, Stdio};
use std::thread;
use std::time::{SystemTime, UNIX_EPOCH};

const ESCAPE_RADIUS_SQ: f64 = 4.0;
const SMOOTH_OFFSET: f64 = 1.0;
const LOG2: f64 = std::f64::consts::LN_2;

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
  pub palette: Vec<Palette>,
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
      palette: default_palettes(),
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
  fn iterate_point(
    &self,
    iterator: &dyn FractalIterator,
    mut z: Complex,
    c: Complex,
    smooth: bool,
  ) -> f64 {
    let mut z_prev = Complex::new(0.0, 0.0);
    let mut i = 0;

    while z.abs_sq() <= ESCAPE_RADIUS_SQ && i < self.max_iterations {
      let temp_z = z;
      z = iterator.iterate(z, z_prev, c);
      z_prev = temp_z;
      i += 1;
    }

    if smooth && i < self.max_iterations {
      let log_zn = z.abs_sq().sqrt().ln().ln();
      i as f64 + SMOOTH_OFFSET - log_zn / LOG2
    } else {
      i as f64
    }
  }

  pub fn generate_image(&self, width: u32, height: u32, smooth: bool) -> Vec<Vec<Rgb<u8>>> {
    let aspect = width as f64 / height as f64;
    let vw = 3.5 / self.scale;
    let vh = vw / aspect;
    let left = self.z.re - vw / 2.0;
    let top = self.z.im - vh / 2.0;

    let iterator: Box<dyn FractalIterator + Send + Sync> = match self.set {
      Set::Mandelbrot => Box::new(MandelbrotIterator { power: self.power }),
      Set::BurningShip => Box::new(BurningShipIterator),
      Set::Julia => Box::new(MandelbrotIterator { power: self.power }),
      Set::Phoenix => Box::new(PhoenixIterator {
        power: self.power,
        c: self.phoenix_c,
        p: self.phoenix_p,
      }),
    };

    (0..height)
      .into_par_iter()
      .map(|y| {
        (0..width)
          .map(|x| {
            let cx = left + x as f64 * vw / width as f64;
            let cy = top + y as f64 * vh / height as f64;
            let (z, c) = match self.set {
              Set::Mandelbrot | Set::BurningShip => (Complex::new(0.0, 0.0), Complex::new(cx, cy)),
              Set::Julia => (
                Complex::new(cx, cy),
                Complex::new(self.julia_c.re, self.julia_c.im),
              ),
              Set::Phoenix => {
                (Complex::new(cy, cx), Complex::new(0.0, 0.0)) // WTF is it rotated ??? TODO
              }
            };

            let iter = self.iterate_point(iterator.as_ref(), z, c, smooth);

            self.colorize(iter)
          })
          .collect()
      })
      .collect()
  }

  fn compute(&mut self, area: Rect) {
    let (w, h) = (area.width as usize, area.height as usize * 2);
    if self.colors.len() == h && self.colors[0].len() == w && !self.need_render {
      return;
    }
    let raw_colors = self.generate_image(w as u32, h as u32, false);
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
      let colors = fractal.generate_image(width, height, true);

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
    progress_tx: std::sync::mpsc::Sender<ProgressEvent>,
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
        let _ = progress_tx.send(ProgressEvent::Finished);
        return Ok(thread_output_path);
      }

      let mut thread_fractal = fractal;
      for frame in 0..total_frames {
        let t = frame as f64 / total_frames as f64;
        let scale = start_scale * (end_scale / start_scale).powf(t);
        thread_fractal.scale = scale;

        let colors = thread_fractal.generate_image(width, height, true);
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
        let _ = progress_tx.send(ProgressEvent::Progress(frame as f64 / total_frames as f64));
      }
      let _ = progress_tx.send(ProgressEvent::Finished);

      if which::which("ffmpeg").is_ok() {
        Command::new("ffmpeg")
          .arg("-framerate")
          .arg(fps.to_string())
          .arg("-i")
          .arg(format!("{}/frame_%04d.png", thread_output_path.display()))
          .arg("-c:v")
          .arg("libx264")
          .arg("-pix_fmt")
          .arg("yuv420p")
          .arg("-crf")
          .arg("18")
          .arg("video.mp4")
          .current_dir(&thread_output_path)
          .stdout(Stdio::null())
          .stderr(Stdio::null())
          .spawn()?;
      }

      Ok(thread_output_path)
    });

    Ok(output_path)
  }

  pub fn colorize(&self, iter: f64) -> Rgb<u8> {
    if iter >= self.max_iterations as f64 {
      return Rgb([0, 0, 0]);
    }

    let palette = &self.palette[self.current_palette];
    let raw_t = iter / palette.cycle_speed;
    let t = raw_t.fract();

    let (r, g, b) = palette.eval(t);
    Rgb([r, g, b])
  }
}
