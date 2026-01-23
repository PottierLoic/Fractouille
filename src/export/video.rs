use crate::app::ProgressEvent;
use crate::complex::Complex;
use crate::fractal::Fractal;
use color_eyre::eyre::{Result, eyre};
use std::io::Write;
use std::time::{SystemTime, UNIX_EPOCH};
use std::{
  fs,
  path::PathBuf,
  process::{Command, Stdio},
  thread,
};

pub fn save_video(
  fractal: &Fractal,
  width: u32,
  height: u32,
  start_scale: f64,
  end_scale: f64,
  zoom_speed: f64,
  progress_tx: std::sync::mpsc::Sender<ProgressEvent>,
) -> Result<PathBuf, String> {
  let fractal = fractal.clone();

  let base_dir = dirs::picture_dir()
    .or_else(dirs::home_dir)
    .ok_or_else(|| "Could not determine user directory".to_string())?;

  let timestamp = SystemTime::now()
    .duration_since(UNIX_EPOCH)
    .unwrap_or_default()
    .as_secs();

  let output_path = base_dir.join("fractouille");

  fs::create_dir_all(&output_path).map_err(|e| e.to_string())?;

  let thread_output_path = output_path.clone();

  thread::spawn(move || -> Result<PathBuf> {
    let fps = 60.0;

    let total_frames = ((end_scale.ln() - start_scale.ln()).abs() / zoom_speed * fps).ceil() as u32;

    if total_frames == 0 {
      let _ = progress_tx.send(ProgressEvent::Finished);
      return Ok(thread_output_path);
    }

    let mut ffmpeg = Command::new("ffmpeg")
      .args([
        "-y",
        "-f",
        "rawvideo",
        "-pixel_format",
        "rgb24",
        "-video_size",
        &format!("{}x{}", width, height),
        "-framerate",
        &fps.to_string(),
        "-i",
        "-",
        "-c:v",
        "libx264",
        "-pix_fmt",
        "yuv420p",
        "-crf",
        "18",
        &format!("{}_zoom.mp4", timestamp),
      ])
      .current_dir(&thread_output_path)
      .stdin(Stdio::piped())
      .stdout(Stdio::null())
      .stderr(Stdio::null())
      .spawn()?;

    let mut stdin = ffmpeg
      .stdin
      .take()
      .ok_or_else(|| eyre!("Failed to open ffmpeg stdin"))?;

    let mut thread_fractal = fractal;

    for frame in 0..total_frames {
      let t = frame as f64 / total_frames as f64;
      thread_fractal.scale = start_scale * (end_scale / start_scale).powf(t);

      let colors = thread_fractal.render_frame(width, height, true);

      let mut frame_buffer = Vec::with_capacity((width * height * 3) as usize);

      for pixel in &colors {
        frame_buffer.push(pixel.0[0]);
        frame_buffer.push(pixel.0[1]);
        frame_buffer.push(pixel.0[2]);
      }

      stdin.write_all(&frame_buffer)?;

      let _ = progress_tx.send(ProgressEvent::Progress(frame as f64 / total_frames as f64));
    }

    drop(stdin);
    ffmpeg.wait()?;

    let _ = progress_tx.send(ProgressEvent::Finished);
    Ok(thread_output_path)
  });

  Ok(output_path)
}
pub fn save_julia_rotation(
  fractal: &Fractal,
  width: u32,
  height: u32,
  duration: f64,
  c_base: Complex,
  angular_speed: f64,
  progress_tx: std::sync::mpsc::Sender<ProgressEvent>,
) -> Result<PathBuf, String> {
  let fractal = fractal.clone();

  let base_dir = dirs::picture_dir()
    .or_else(dirs::home_dir)
    .ok_or_else(|| "Could not determine user directory".to_string())?;

  let timestamp = SystemTime::now()
    .duration_since(UNIX_EPOCH)
    .unwrap_or_default()
    .as_secs();

  let output_path = base_dir.join("fractouille");
  fs::create_dir_all(&output_path).map_err(|e| e.to_string())?;
  let thread_output_path = output_path.clone();

  thread::spawn(move || -> Result<PathBuf> {
    let fps = 60.0;
    let total_frames = (duration * fps).ceil() as u32;

    if total_frames == 0 {
      let _ = progress_tx.send(ProgressEvent::Finished);
      return Ok(thread_output_path);
    }

    let initial_radius = c_base.abs();
    let initial_angle = c_base.im.atan2(c_base.re);

    let angle_per_frame = angular_speed / fps;

    let mut ffmpeg = Command::new("ffmpeg")
      .args([
        "-y",
        "-f",
        "rawvideo",
        "-pixel_format",
        "rgb24",
        "-video_size",
        &format!("{}x{}", width, height),
        "-framerate",
        &fps.to_string(),
        "-i",
        "-",
        "-c:v",
        "libx264",
        "-pix_fmt",
        "yuv420p",
        "-crf",
        "18",
        &format!("{}_rot.mp4", timestamp),
      ])
      .current_dir(&thread_output_path)
      .stdin(Stdio::piped())
      .stdout(Stdio::null())
      .stderr(Stdio::null())
      .spawn()?;

    let mut stdin = ffmpeg
      .stdin
      .take()
      .ok_or_else(|| eyre!("Failed to open ffmpeg stdin"))?;

    let mut thread_fractal = fractal;

    for frame in 0..total_frames {
      let current_angle = initial_angle + angle_per_frame * frame as f64;

      let current_c = Complex {
        re: initial_radius.re * current_angle.cos(),
        im: initial_radius.im * current_angle.sin(),
      };

      thread_fractal.julia_c = current_c;

      let colors = thread_fractal.render_frame(width, height, true);

      let mut frame_buffer = Vec::with_capacity((width * height * 3) as usize);

      for pixel in &colors {
        frame_buffer.push(pixel.0[0]);
        frame_buffer.push(pixel.0[1]);
        frame_buffer.push(pixel.0[2]);
      }

      stdin.write_all(&frame_buffer)?;

      let _ = progress_tx.send(ProgressEvent::Progress(frame as f64 / total_frames as f64));
    }

    drop(stdin);
    ffmpeg.wait()?;

    let _ = progress_tx.send(ProgressEvent::Finished);
    Ok(thread_output_path)
  });

  Ok(output_path)
}
