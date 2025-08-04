use crate::App;
use crate::fractal::Fractal;
use color_eyre::Result;

#[derive(Debug)]
enum Command {
  Move(f64, f64, Option<f64>),
  Reset,
  Screenshot(Option<u32>, Option<u32>),
  Help,
  Quit,

  Complex(f64, f64),
  Power(f64),
  Iterations(u32),
  Zoom(f64),

  Set(String),

  Unknown(String),
}

pub struct CommandProcessor;

impl CommandProcessor {
  fn parse(command: &str) -> Command {
    let parts: Vec<&str> = command.split_whitespace().collect();

    if parts.is_empty() {
      return Command::Unknown("Empty command".to_string());
    }

    match parts[0] {
      "move" => {
        if parts.len() < 3 || parts.len() > 4 {
          return Command::Unknown(format!(
            "Usage: move <x> <y> <zoom>. Got {} arguments",
            parts.len() - 1
          ));
        }
        let x = match parts[1].parse::<f64>() {
          Ok(val) => val,
          Err(_) => return Command::Unknown(format!("Invalid x coordinate: {}", parts[1])),
        };
        let y = match parts[2].parse::<f64>() {
          Ok(val) => val,
          Err(_) => return Command::Unknown(format!("Invalid y coordinate: {}", parts[2])),
        };
        let zoom = if parts.len() == 4 {
          match parts[3].parse::<f64>() {
            Ok(val) => {
              if val <= 0.0 {
                return Command::Unknown("Zoom must be positive".to_string());
              }
              Some(val)
            }
            Err(_) => return Command::Unknown(format!("Invalid zoom level: {}", parts[3])),
          }
        } else {
          None
        };
        Command::Move(x, y, zoom)
      }
      "reset" => Command::Reset,
      "screenshot" => {
        if parts.len() == 3 {
          let width = match parts[1].parse::<u32>() {
            Ok(val) => val,
            Err(_) => return Command::Unknown(format!("Invalid width: {}", parts[1])),
          };
          let height = match parts[2].parse::<u32>() {
            Ok(val) => val,
            Err(_) => return Command::Unknown(format!("Invalid height: {}", parts[2])),
          };
          Command::Screenshot(Some(width), Some(height))
        } else {
          Command::Screenshot(None, None)
        }
      }
      "h" => Command::Help,
      "help" => Command::Help,
      "q" => Command::Quit,
      "quit" => Command::Quit,
      "complex" => {
        if parts.len() != 3 {
          return Command::Unknown(format!(
            "Usage: complex <real> <imaginary>. Got {} arguments",
            parts.len() - 1
          ));
        }
        let real = match parts[1].parse::<f64>() {
          Ok(val) => val,
          Err(_) => return Command::Unknown(format!("Invalid real part: {}", parts[1])),
        };
        let imag = match parts[2].parse::<f64>() {
          Ok(val) => val,
          Err(_) => return Command::Unknown(format!("Invalid imaginary part: {}", parts[2])),
        };
        Command::Complex(real, imag)
      }
      "power" => {
        if parts.len() != 2 {
          return Command::Unknown(format!(
            "Usage: power <value>. Got {} arguments",
            parts.len() - 1
          ));
        }
        match parts[1].parse::<f64>() {
          Ok(val) => Command::Power(val),
          Err(_) => Command::Unknown(format!("Invalid power value: {}", parts[1])),
        }
      }
      "iterations" => {
        if parts.len() != 2 {
          return Command::Unknown(format!(
            "Usage: iterations <count>. Got {} arguments",
            parts.len() - 1
          ));
        }

        match parts[1].parse::<u32>() {
          Ok(val) => Command::Iterations(val),
          Err(_) => Command::Unknown(format!("Invalid iterations count: {}", parts[1])),
        }
      }
      "zoom" => {
        if parts.len() != 2 {
          return Command::Unknown(format!(
            "Usage: zoom <factor>. Got {} arguments",
            parts.len() - 1
          ));
        }

        match parts[1].parse::<f64>() {
          Ok(val) => {
            if val <= 0.0 {
              Command::Unknown("Zoom factor must be positive".to_string())
            } else {
              Command::Zoom(val)
            }
          }
          Err(_) => Command::Unknown(format!("Invalid zoom factor: {}", parts[1])),
        }
      }
      "set" => {
        if parts.len() != 2 {
          Command::Unknown(format!(
            "Usage: set <set>. Got {} arguments",
            parts.len() - 1
          ))
        } else if parts[1] == "mandelbrot" || parts[1] == "julia" || parts[1] == "burningship" {
          Command::Set(parts[1].to_string())
        } else {
          Command::Unknown(format!("Unknown set: {}", parts[1]))
        }
      }

      cmd => Command::Unknown(format!("Unknown command: {}", cmd)),
    }
  }

  pub fn execute(app: &mut App) -> Result<String> {
    match Self::parse(&app.command_string) {
      Command::Move(x, y, zoom) => {
        app.fractal.z = (x, y);
        if let Some(zoom) = zoom {
          app.fractal.scale = zoom;
        }
        app.fractal.need_render = true;
        Ok("Position and zoom updated".to_string())
      }
      Command::Reset => {
        app.fractal = Fractal::default();
        app.fractal.need_render = true;
        Ok("Fractal reset to default state".to_string())
      }
      Command::Screenshot(width, height) => match app.fractal.save_screenshot(width, height) {
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
      Command::Complex(real, imag) => {
        app.fractal.julia_constant = (real, imag);
        app.fractal.need_render = true;
        Ok(format!(
          "Julia set constant updated to {} + {}i",
          real, imag
        ))
      }
      Command::Power(power) => {
        app.fractal.power = power;
        app.fractal.need_render = true;
        Ok(format!("Fractal power updated to {}", power))
      }
      Command::Iterations(count) => {
        app.fractal.max_iterations = count;
        app.fractal.need_render = true;
        Ok(format!("Max iterations updated to {}", count))
      }
      Command::Zoom(factor) => {
        app.fractal.scale *= factor;
        app.fractal.need_render = true;
        Ok(format!("Zoomed by factor of {}", factor))
      }
      Command::Set(set) => {
        match set.as_str() {
          "mandelbrot" => app.fractal.set = crate::fractal::Set::Mandelbrot,
          "julia" => app.fractal.set = crate::fractal::Set::Julia,
          "burningship" => app.fractal.set = crate::fractal::Set::BurningShip,
          "phoenix" => app.fractal.set = crate::fractal::Set::Phoenix,
          &_ => (),
        }
        app.fractal.need_render = true;
        Ok(format!("Switched to {} set", set).to_string())
      }
      Command::Unknown(msg) => Ok(msg),
    }
  }
}
