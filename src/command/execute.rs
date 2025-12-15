use crate::app::App;
use crate::command::Command;
use crate::complex::Complex;
use crate::export::video::save_julia_rotation;
use crate::export::{save_image, save_video};
use crate::fractal::Fractal;
use crate::palette::{InterpolationMode, Palette};
use color_eyre::Result;

pub fn execute_command(app: &mut App, cmd: Command) -> Result<String> {
  match cmd {
    Command::Move(x, y, zoom) => {
      app.fractal.z.re = x;
      app.fractal.z.im = y;
      if let Some(zoom) = zoom {
        app.fractal.scale = zoom;
      }
      app.fractal_view.need_render = true;
      Ok("Position and zoom updated".to_string())
    }
    Command::Reset => {
      app.fractal = Fractal::default();
      app.fractal_view.need_render = true;
      Ok("Fractal reset to default state".to_string())
    }
    Command::Screenshot(width, height) => match save_image(&app.fractal, width, height) {
      Ok(path) => Ok(format!("Screenshot saved in {}", path.display())),
      Err(e) => Ok(format!("Failed to save screenshot: {}", e)),
    },
    Command::Help => {
      app.show_extended_menu = !app.show_extended_menu;
      Ok("Menu extended".to_string())
    }
    Command::Quit => {
      app.quit_requested = true;
      Ok("Bye!".to_string())
    }
    Command::Julia(real, imag) => {
      app.fractal.julia_c.re = real;
      app.fractal.julia_c.im = imag;
      app.fractal_view.need_render = true;
      Ok(format!(
        "Julia set constant updated to {} + {}i",
        real, imag
      ))
    }
    Command::PhoenixC(real) => {
      app.fractal.phoenix_c.re = real;
      Ok(format!("Phoenix set constant C updated to {}", real))
    }
    Command::PhoenixP(real, imag) => {
      app.fractal.phoenix_p.re = real;
      app.fractal.phoenix_p.im = imag;
      Ok(format!(
        "Phoenix set constant P updated to {} + {}i",
        real, imag
      ))
    }
    Command::Power(power) => {
      app.fractal.power = power;
      app.fractal_view.need_render = true;
      Ok(format!("Fractal power updated to {}", power))
    }
    Command::Iterations(count) => {
      app.fractal.max_iterations = count;
      app.fractal_view.need_render = true;
      Ok(format!("Max iterations updated to {}", count))
    }
    Command::Zoom(zoom_level) => {
      app.fractal.scale = zoom_level;
      app.fractal_view.need_render = true;
      Ok(format!("Zoomed to {}", zoom_level))
    }
    Command::Set(set) => {
      match set.as_str() {
        "mandelbrot" => app.fractal.set = crate::fractal::Set::Mandelbrot,
        "julia" => app.fractal.set = crate::fractal::Set::Julia,
        "burningship" => app.fractal.set = crate::fractal::Set::BurningShip,
        "phoenix" => app.fractal.set = crate::fractal::Set::Phoenix,
        &_ => (),
      }
      app.fractal_view.need_render = true;
      Ok(format!("Switched to {} set", set).to_string())
    }
    Command::Record(width, height, start, end, speed) => {
      let (tx, rx) = std::sync::mpsc::channel();
      app.progress_rx = Some(rx);
      app.show_record_popup = true;
      app.record_progress = 0.0;
      match save_video(&app.fractal, width, height, start, end, speed, tx) {
        Ok(path) => Ok(format!("Record frames saved in {}", path.display())),
        Err(e) => Ok(format!("Failed to save record frames: {}", e)),
      }
    }
    Command::RecordJulia(width, height, duration, real, imag, speed) => {
      let (tx, rx) = std::sync::mpsc::channel();
      app.progress_rx = Some(rx);
      app.show_record_popup = true;
      app.record_progress = 0.0;
      match save_julia_rotation(
        &app.fractal,
        width,
        height,
        duration,
        Complex { re: real, im: imag },
        speed,
        tx,
      ) {
        Ok(path) => Ok(format!("Record frames saved in {}", path.display())),
        Err(e) => Ok(format!("Failed to save record frames: {}", e)),
      }
    }
    Command::CycleSpeed(cycle) => {
      app.fractal.palette[app.fractal.current_palette].cycle_speed = cycle;
      app.fractal_view.need_render = true;
      Ok(format!("Color cycle updated to {}", cycle))
    }
    Command::PaletteCreate(colors) => {
      let palette = Palette::new(colors, InterpolationMode::Linear, 100.0);
      app.fractal.palette.push(palette);
      Ok(format!(
        "Palette inserted at index {}",
        app.fractal.palette.len() - 1
      ))
    }
    Command::PaletteUse(index) => {
      if app.fractal.palette.len() <= index {
        return Ok("Palette index out of range".to_string());
      }
      app.fractal.current_palette = index;
      Ok(format!("Palette updated to {}", index))
    }
    Command::PaletteDelete(index) => {
      if app.fractal.palette.len() == 1 {
        return Ok("Cannot delete last palette".to_string());
      }
      if app.fractal.palette.len() <= index {
        return Ok("Palette index out of range".to_string());
      }
      app.fractal.palette.remove(index);
      Ok("Palette deleted".to_string())
    }
    Command::PaletteMode(mode) => {
      app.fractal.palette[app.fractal.current_palette].interpolation = mode;
      app.fractal_view.need_render = true;
      Ok(format!(
        "Change interpolation mode of palette {} to {:?}",
        app.fractal.current_palette, app.fractal.palette[app.fractal.current_palette].interpolation
      ))
    }
    Command::Unknown(msg) => Ok(msg),
  }
}
