use crate::fractal::Fractal;
use color_eyre::eyre::Result;
use image::RgbImage;
use std::time::{SystemTime, UNIX_EPOCH};
use std::{fs, path::PathBuf, thread};

pub fn save_image(
  fractal: &Fractal,
  width: Option<u32>,
  height: Option<u32>,
) -> Result<PathBuf, String> {
  let fractal = fractal.clone();
  let width = width.unwrap_or(1920);
  let height = height.unwrap_or(1080);

  let base_dir = dirs::picture_dir()
    .or_else(dirs::home_dir)
    .ok_or_else(|| "Could not determine user directory".to_string())?;

  let screenshots_dir = base_dir.join("fractouille");
  fs::create_dir_all(&screenshots_dir).map_err(|e| e.to_string())?;

  let thread_screenshots_dir = screenshots_dir.clone();

  thread::spawn(move || -> Result<PathBuf> {
    let mut img = RgbImage::new(width, height);
    let colors = fractal.render_frame(width, height, true);

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
      crate::fractal::Set::Mandelbrot => "mandelbrot",
      crate::fractal::Set::Julia => "julia",
      crate::fractal::Set::BurningShip => "burningship",
      crate::fractal::Set::Phoenix => "phoenix",
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
