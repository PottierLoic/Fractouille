use crate::App;
use crate::fractal::Fractal;
use crate::palettes::{InterpolationMode, Palette};
use color_eyre::Result;

#[derive(Debug)]
enum Command {
  Move(f64, f64, Option<f64>),
  Reset,
  Screenshot(Option<u32>, Option<u32>),
  Help,
  Quit,
  Julia(f64, f64),
  PhoenixC(f64),
  PhoenixP(f64, f64),
  Power(f64),
  Iterations(u32),
  Zoom(f64),
  Set(String),
  Record(u32, u32, f64, f64, f64),
  ColorCycle(f64),
  PaletteCreate(Vec<(u8, u8, u8)>),
  PaletteUse(usize),
  PaletteDelete(usize),
  PaletteChangeMode(InterpolationMode),
  Unknown(String),
}

static COMMAND_NAMES: &[&str] = &[
  "move <x> <y> [zoom]",
  "reset",
  "screenshot [width] [height]",
  "help",
  "quit",
  "julia <real> <imag>",
  "phoenix_c <real>",
  "phoenix_p <real> <imag>",
  "power <value>",
  "iterations <count>",
  "zoom <level>",
  "set <mandelbrot|julia|burningship>",
  "record <w> <h> <start> <end> <speed>",
  "colorcycle <value>",
  "palette create <r0> <g0> <b0> ... <rn> <gn> <bn>",
  "palette use <index>",
  "palette delete <index>",
  "palette changemode <linear|cosine>",
];

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
      "julia" => {
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
        Command::Julia(real, imag)
      }
      "phoenix_c" => {
        if parts.len() != 2 {
          return Command::Unknown(format!(
            "Usage: phoenix_c <real>. Got {} arguments",
            parts.len() - 1
          ));
        }
        let real = match parts[1].parse::<f64>() {
          Ok(val) => val,
          Err(_) => return Command::Unknown(format!("Invalid real part: {}", parts[1])),
        };
        Command::PhoenixC(real)
      }
      "phoenix_p" => {
        if parts.len() != 3 {
          return Command::Unknown(format!(
            "Usage: phoenix_p <real> <imaginary>. Got {} arguments",
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
        Command::PhoenixP(real, imag)
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
            "Usage: zoom <zoom_level>. Got {} arguments",
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
      "record" => {
        if parts.len() != 6 {
          Command::Unknown(format!(
            "Usage: record <width> <height> <start_scale> <end_scale> <speed>. Got {} arguments",
            parts.len() - 1
          ))
        } else {
          let width = match parts[1].parse::<u32>() {
            Ok(val) => val,
            Err(_) => return Command::Unknown(format!("Invalid width: {}", parts[1])),
          };
          let height = match parts[2].parse::<u32>() {
            Ok(val) => val,
            Err(_) => return Command::Unknown(format!("Invalid height: {}", parts[2])),
          };
          let start = match parts[3].parse::<f64>() {
            Ok(val) => val,
            Err(_) => return Command::Unknown(format!("Invalid start_scale: {}", parts[3])),
          };
          let end = match parts[4].parse::<f64>() {
            Ok(val) => val,
            Err(_) => return Command::Unknown(format!("Invalid end_scale: {}", parts[4])),
          };
          let speed = match parts[5].parse::<f64>() {
            Ok(val) => val,
            Err(_) => return Command::Unknown(format!("Invalid speed: {}", parts[5])),
          };
          Command::Record(width, height, start, end, speed)
        }
      }
      "colorcycle" => {
        if parts.len() != 2 {
          Command::Unknown(format!(
            "Usage: colorcycle <cycle value>. Got {} arguments",
            parts.len() - 1
          ))
        } else {
          match parts[1].parse::<f64>() {
            Ok(val) => Command::ColorCycle(val),
            Err(_) => Command::Unknown(format!("Invalid cycle value: {}", parts[1])),
          }
        }
      }
      "palette" => {
        if parts.len() < 2 {
          Command::Unknown(format!(
            "Usage: palette <create|use|delete> <color>. Got {} arguments",
            parts.len() - 1
          ))
        } else {
          match parts[1] {
            "create" => {
              if (parts.len() - 2) % 3 != 0 {
                return Command::Unknown(format!(
                  "Usage: palette create <r0> <g0> <b0> ... <rn> <gn> <bn>. Got {} arguments",
                  parts.len() - 2
                ));
              }

              let mut colors = Vec::new();
              for chunk in parts[2..].chunks(3) {
                let r = match chunk[0].parse::<u8>() {
                  Ok(val) => val,
                  Err(_) => return Command::Unknown(format!("Invalid red color: {}", chunk[0])),
                };
                let g = match chunk[1].parse::<u8>() {
                  Ok(val) => val,
                  Err(_) => return Command::Unknown(format!("Invalid red color: {}", chunk[1])),
                };
                let b = match chunk[2].parse::<u8>() {
                  Ok(val) => val,
                  Err(_) => return Command::Unknown(format!("Invalid red color: {}", chunk[2])),
                };

                colors.push((r, g, b));
              }
              Command::PaletteCreate(colors)
            }
            "use" => {
              if parts.len() != 3 {
                Command::Unknown(format!(
                  "Usage: palette use <index>. Got {} arguments",
                  parts.len() - 1
                ))
              } else {
                let index = match parts[2].parse::<usize>() {
                  Ok(val) => val,
                  Err(_) => {
                    return Command::Unknown(format!("Invalid palette index: {}", parts[2]));
                  }
                };
                Command::PaletteUse(index)
              }
            }
            "delete" => {
              if parts.len() != 3 {
                Command::Unknown(format!(
                  "Usage: palette delete <index>. Got {} arguments",
                  parts.len() - 1
                ))
              } else {
                let index = match parts[2].parse::<usize>() {
                  Ok(val) => val,
                  Err(_) => {
                    return Command::Unknown(format!("Invalid palette index: {}", parts[2]));
                  }
                };
                Command::PaletteDelete(index)
              }
            }
            "changemode" => {
              if parts.len() != 3 {
                Command::Unknown(format!(
                  "Usage: palette changemode <linear|cosine>. Got {} arguments",
                  parts.len() - 1
                ))
              } else {
                match parts[2] {
                  "linear" => Command::PaletteChangeMode(InterpolationMode::Linear),
                  "cosine" => Command::PaletteChangeMode(InterpolationMode::Cosine),
                  _ => {
                    Command::Unknown(format!("Unknown palette interpolation mode: {}", parts[3]))
                  }
                }
              }
            }
            _ => Command::Unknown(format!("Unknown palette command: {}", parts[1])),
          }
        }
      }

      cmd => Command::Unknown(format!("Unknown command: {}", cmd)),
    }
  }

  pub fn execute(app: &mut App) -> Result<String> {
    match Self::parse(&app.command_string) {
      Command::Move(x, y, zoom) => {
        app.fractal.z.re = x;
        app.fractal.z.im = y;
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
      Command::Julia(real, imag) => {
        app.fractal.julia_c.re = real;
        app.fractal.julia_c.im = imag;
        app.fractal.need_render = true;
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
        app.fractal.need_render = true;
        Ok(format!("Fractal power updated to {}", power))
      }
      Command::Iterations(count) => {
        app.fractal.max_iterations = count;
        app.fractal.need_render = true;
        Ok(format!("Max iterations updated to {}", count))
      }
      Command::Zoom(zoom_level) => {
        app.fractal.scale = zoom_level;
        app.fractal.need_render = true;
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
        app.fractal.need_render = true;
        Ok(format!("Switched to {} set", set).to_string())
      }
      Command::Record(width, height, start, end, speed) => {
        let (tx, rx) = std::sync::mpsc::channel();
        app.progress_rx = Some(rx);
        app.show_record_popup = true;
        app.record_progress = 0.0;
        match app
          .fractal
          .record_zoom(width, height, start, end, speed, tx)
        {
          Ok(path) => Ok(format!("Record frames saved in {}", path.display())),
          Err(e) => Ok(format!("Failed to save record frames: {}", e)),
        }
      }
      Command::ColorCycle(cycle) => {
        app.fractal.palette[app.fractal.current_palette].cycle_speed = cycle;
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
      Command::PaletteChangeMode(mode) => {
        app.fractal.palette[app.fractal.current_palette].interpolation = mode;
        Ok(format!(
          "Change interpolation mode of palette {} to {:?}",
          app.fractal.current_palette,
          app.fractal.palette[app.fractal.current_palette].interpolation
        ))
      }
      Command::Unknown(msg) => Ok(msg),
    }
  }

  pub fn find_command_match(command: &str) -> Option<&str> {
    let cmd = COMMAND_NAMES
      .iter()
      .find(|name| name.starts_with(command))?;
    Some(&cmd[command.len()..])
  }

  pub fn find_command_autocompletion(command: &str) -> Option<&str> {
    let cmd = COMMAND_NAMES
      .iter()
      .find(|name| name.starts_with(command))?;
    cmd.split_whitespace().next()
  }
}
