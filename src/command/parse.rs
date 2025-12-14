use crate::command::Command;
use crate::palette::InterpolationMode;

pub fn parse_command(input: &str) -> Command {
  let parts: Vec<&str> = input.split_whitespace().collect();
  if parts.is_empty() {
    return Command::Unknown("Empty command".to_string());
  }

  match parts[0] {
    "move" => parse_move(&parts),
    "reset" => Command::Reset,
    "screenshot" => parse_screenshot(&parts),
    "help" | "h" => Command::Help,
    "quit" | "q" => Command::Quit,
    "julia" => parse_julia(&parts),
    "phoenix_c" => parse_phoenix_c(&parts),
    "phoenix_p" => parse_phoenix_p(&parts),
    "power" => parse_power(&parts),
    "iterations" => parse_iterations(&parts),
    "zoom" => parse_zoom(&parts),
    "set" => parse_set(&parts),
    "record" => parse_record(&parts),
    "record_julia" => parse_record_julia(&parts),
    "cycle_speed" => parse_cycle_speed(&parts),
    "palette" => parse_palette(&parts),

    other => Command::Unknown(format!("Unknown command: {}", other)),
  }
}

pub fn parse_move(parts: &[&str]) -> Command {
  if parts.len() < 3 || parts.len() > 4 {
    return Command::Unknown(format!(
      "Usage: move <x> <y> <zoom>. Got {} arguments",
      parts.len() - 1
    ));
  }

  let x = match parts[1].parse::<f64>() {
    Ok(v) => v,
    Err(_) => return Command::Unknown(format!("Invalid x coordinate: {}", parts[1])),
  };

  let y = match parts[2].parse::<f64>() {
    Ok(v) => v,
    Err(_) => return Command::Unknown(format!("Invalid y coordinate: {}", parts[2])),
  };

  let zoom = if parts.len() == 4 {
    match parts[3].parse::<f64>() {
      Ok(v) => {
        if v <= 0.0 {
          return Command::Unknown("Zoom must be positive".into());
        }
        Some(v)
      }
      Err(_) => return Command::Unknown(format!("Invalid zoom level: {}", parts[3])),
    }
  } else {
    None
  };

  Command::Move(x, y, zoom)
}

pub fn parse_screenshot(parts: &[&str]) -> Command {
  if parts.len() == 3 {
    let width = match parts[1].parse::<u32>() {
      Ok(v) => v,
      Err(_) => return Command::Unknown(format!("Invalid width: {}", parts[1])),
    };

    let height = match parts[2].parse::<u32>() {
      Ok(v) => v,
      Err(_) => return Command::Unknown(format!("Invalid height: {}", parts[2])),
    };

    Command::Screenshot(Some(width), Some(height))
  } else {
    Command::Screenshot(None, None)
  }
}

pub fn parse_julia(parts: &[&str]) -> Command {
  if parts.len() != 3 {
    return Command::Unknown(format!(
      "Usage: complex <real> <imaginary>. Got {} arguments",
      parts.len() - 1
    ));
  }

  let real = match parts[1].parse::<f64>() {
    Ok(v) => v,
    Err(_) => return Command::Unknown(format!("Invalid real part: {}", parts[1])),
  };

  let imag = match parts[2].parse::<f64>() {
    Ok(v) => v,
    Err(_) => return Command::Unknown(format!("Invalid imaginary part: {}", parts[2])),
  };

  Command::Julia(real, imag)
}

pub fn parse_phoenix_c(parts: &[&str]) -> Command {
  if parts.len() != 2 {
    return Command::Unknown(format!(
      "Usage: phoenix_c <real>. Got {} arguments",
      parts.len() - 1
    ));
  }

  let real = match parts[1].parse::<f64>() {
    Ok(v) => v,
    Err(_) => return Command::Unknown(format!("Invalid real part: {}", parts[1])),
  };

  Command::PhoenixC(real)
}

pub fn parse_phoenix_p(parts: &[&str]) -> Command {
  if parts.len() != 3 {
    return Command::Unknown(format!(
      "Usage: phoenix_p <real> <imaginary>. Got {} arguments",
      parts.len() - 1
    ));
  }

  let real = match parts[1].parse::<f64>() {
    Ok(v) => v,
    Err(_) => return Command::Unknown(format!("Invalid real part: {}", parts[1])),
  };

  let imag = match parts[2].parse::<f64>() {
    Ok(v) => v,
    Err(_) => return Command::Unknown(format!("Invalid imaginary part: {}", parts[2])),
  };

  Command::PhoenixP(real, imag)
}

pub fn parse_power(parts: &[&str]) -> Command {
  if parts.len() != 2 {
    return Command::Unknown(format!(
      "Usage: power <value>. Got {} arguments",
      parts.len() - 1
    ));
  }

  match parts[1].parse::<f64>() {
    Ok(v) => Command::Power(v),
    Err(_) => Command::Unknown(format!("Invalid power value: {}", parts[1])),
  }
}

pub fn parse_iterations(parts: &[&str]) -> Command {
  if parts.len() != 2 {
    return Command::Unknown(format!(
      "Usage: iterations <count>. Got {} arguments",
      parts.len() - 1
    ));
  }

  match parts[1].parse::<u32>() {
    Ok(v) => Command::Iterations(v),
    Err(_) => Command::Unknown(format!("Invalid iterations count: {}", parts[1])),
  }
}

pub fn parse_zoom(parts: &[&str]) -> Command {
  if parts.len() != 2 {
    return Command::Unknown(format!(
      "Usage: zoom <zoom_level>. Got {} arguments",
      parts.len() - 1
    ));
  }

  match parts[1].parse::<f64>() {
    Ok(v) => {
      if v <= 0.0 {
        Command::Unknown("Zoom factor must be positive".into())
      } else {
        Command::Zoom(v)
      }
    }
    Err(_) => Command::Unknown(format!("Invalid zoom factor: {}", parts[1])),
  }
}

pub fn parse_set(parts: &[&str]) -> Command {
  if parts.len() != 2 {
    return Command::Unknown(format!(
      "Usage: set <set>. Got {} arguments",
      parts.len() - 1
    ));
  }

  match parts[1] {
    "mandelbrot" | "julia" | "burningship" => Command::Set(parts[1].into()),
    _ => Command::Unknown(format!("Unknown set: {}", parts[1])),
  }
}

pub fn parse_record(parts: &[&str]) -> Command {
  if parts.len() != 6 {
    return Command::Unknown(format!(
      "Usage: record <width> <height> <start_scale> <end_scale> <speed>. Got {} arguments",
      parts.len() - 1
    ));
  }

  let width = match parts[1].parse::<u32>() {
    Ok(v) => v,
    Err(_) => return Command::Unknown(format!("Invalid width: {}", parts[1])),
  };

  let height = match parts[2].parse::<u32>() {
    Ok(v) => v,
    Err(_) => return Command::Unknown(format!("Invalid height: {}", parts[2])),
  };

  let start = match parts[3].parse::<f64>() {
    Ok(v) => v,
    Err(_) => return Command::Unknown(format!("Invalid start_scale: {}", parts[3])),
  };

  let end = match parts[4].parse::<f64>() {
    Ok(v) => v,
    Err(_) => return Command::Unknown(format!("Invalid end_scale: {}", parts[4])),
  };

  let speed = match parts[5].parse::<f64>() {
    Ok(v) => v,
    Err(_) => return Command::Unknown(format!("Invalid speed: {}", parts[5])),
  };

  Command::Record(width, height, start, end, speed)
}

pub fn parse_record_julia(parts: &[&str]) -> Command {
  if parts.len() != 7 {
    return Command::Unknown(format!(
      "Usage: record_julia <width> <height> <re> <im> <speed>. Got {} arguments",
      parts.len() - 1
    ));
  }

  let width = match parts[1].parse::<u32>() {
    Ok(v) => v,
    Err(_) => return Command::Unknown(format!("Invalid width: {}", parts[1])),
  };

  let height = match parts[2].parse::<u32>() {
    Ok(v) => v,
    Err(_) => return Command::Unknown(format!("Invalid height: {}", parts[2])),
  };

  let duration = match parts[3].parse::<f64>() {
    Ok(v) => v,
    Err(_) => return Command::Unknown(format!("Invalid duration: {}", parts[2])),
  };

  let re = match parts[4].parse::<f64>() {
    Ok(v) => v,
    Err(_) => return Command::Unknown(format!("Invalid re: {}", parts[3])),
  };

  let im = match parts[5].parse::<f64>() {
    Ok(v) => v,
    Err(_) => return Command::Unknown(format!("Invalid im: {}", parts[4])),
  };

  let speed = match parts[6].parse::<f64>() {
    Ok(v) => v,
    Err(_) => return Command::Unknown(format!("Invalid speed: {}", parts[5])),
  };

  Command::RecordJulia(width, height, duration, re, im, speed)
}

pub fn parse_cycle_speed(parts: &[&str]) -> Command {
  if parts.len() != 2 {
    return Command::Unknown(format!(
      "Usage: cycle_speed <cycle value>. Got {} arguments",
      parts.len() - 1
    ));
  }

  match parts[1].parse::<f64>() {
    Ok(v) => Command::CycleSpeed(v),
    Err(_) => Command::Unknown(format!("Invalid cycle value: {}", parts[1])),
  }
}

pub fn parse_palette(parts: &[&str]) -> Command {
  if parts.len() < 2 {
    return Command::Unknown(format!(
      "Usage: palette <create|use|delete> <color>. Got {} arguments",
      parts.len() - 1
    ));
  }

  match parts[1] {
    "create" => parse_palette_create(parts),
    "use" => parse_palette_use(parts),
    "delete" => parse_palette_delete(parts),
    "mode" => parse_palette_mode(parts),
    _ => Command::Unknown(format!("Unknown palette command: {}", parts[1])),
  }
}

pub fn parse_palette_create(parts: &[&str]) -> Command {
  if (parts.len() - 2) % 3 != 0 {
    return Command::Unknown(format!(
      "Usage: palette create <r0> <g0> <b0> ... <rn> <gn> <bn>. Got {} arguments",
      parts.len() - 2
    ));
  }

  let mut colors = Vec::new();

  for chunk in parts[2..].chunks(3) {
    let r = match chunk[0].parse::<u8>() {
      Ok(v) => v,
      Err(_) => return Command::Unknown(format!("Invalid red color: {}", chunk[0])),
    };
    let g = match chunk[1].parse::<u8>() {
      Ok(v) => v,
      Err(_) => return Command::Unknown(format!("Invalid red color: {}", chunk[1])),
    };
    let b = match chunk[2].parse::<u8>() {
      Ok(v) => v,
      Err(_) => return Command::Unknown(format!("Invalid red color: {}", chunk[2])),
    };

    colors.push((r, g, b));
  }

  Command::PaletteCreate(colors)
}

pub fn parse_palette_use(parts: &[&str]) -> Command {
  if parts.len() != 3 {
    return Command::Unknown(format!(
      "Usage: palette use <index>. Got {} arguments",
      parts.len() - 1
    ));
  }

  let index = match parts[2].parse::<usize>() {
    Ok(v) => v,
    Err(_) => return Command::Unknown(format!("Invalid palette index: {}", parts[2])),
  };

  Command::PaletteUse(index)
}

pub fn parse_palette_delete(parts: &[&str]) -> Command {
  if parts.len() != 3 {
    return Command::Unknown(format!(
      "Usage: palette delete <index>. Got {} arguments",
      parts.len() - 1
    ));
  }

  let index = match parts[2].parse::<usize>() {
    Ok(v) => v,
    Err(_) => return Command::Unknown(format!("Invalid palette index: {}", parts[2])),
  };

  Command::PaletteDelete(index)
}

pub fn parse_palette_mode(parts: &[&str]) -> Command {
  if parts.len() != 3 {
    return Command::Unknown(format!(
      "Usage: palette mode <linear|cosine|hsv|hsv_cyclic>. Got {} arguments",
      parts.len() - 1
    ));
  }

  match parts[2] {
    "linear" => Command::PaletteMode(InterpolationMode::Linear),
    "cosine" => Command::PaletteMode(InterpolationMode::Cosine),
    "hsv" => Command::PaletteMode(InterpolationMode::Hsv),
    "hsv_cyclic" => Command::PaletteMode(InterpolationMode::HsvCyclic),
    _ => Command::Unknown(format!("Unknown palette interpolation mode: {}", parts[2])),
  }
}
