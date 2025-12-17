use crate::app::App;
use crate::command::{execute_command, parse_command};
use crate::fractal::Set;
use crate::palette::InterpolationMode;
use crossterm::{
  event::{self, Event, KeyCode},
  terminal::{disable_raw_mode, enable_raw_mode},
};
use image::Rgb;
use std::io::{self, Write};
use std::process::exit;
use std::time::Duration;

fn clear_terminal() {
  print!("\x1b[2J\x1b[H");
  io::stdout().flush().unwrap();
}

fn to_sixel(v: u8) -> u32 {
  v as u32 * 100 / 255
}

pub fn start_sixel_rendering(width: u32, height: u32) {
  let mut app = App::default();
  let mut set_selected = false;
  for palette in &mut app.fractal.palette {
    palette.interpolation = InterpolationMode::None;
  }

  while !set_selected {
    clear_terminal();
    print!("Choose a fractal set to render:\n1. Mandelbrot\n2. Julia\n3. Burning Ship\n4. Quit\n>");
    io::stdout().flush().unwrap();
    let mut input = String::new();
    if io::stdin().read_line(&mut input).is_err() {
      continue;
    }

    match input.trim() {
      "1" => {
        app.fractal.set = Set::Mandelbrot;
        set_selected = true;
      }
      "2" => {
        app.fractal.set = Set::Julia;
        set_selected = true;
      }
      "3" => {
        app.fractal.set = Set::BurningShip;
        set_selected = true;
      }
      "4" => exit(0),
      _ => continue,
    }
    clear_terminal();
  }

  let mut command_mode = false;
  let mut command_string = String::new();

  enable_raw_mode().unwrap();
  let mut first_render = true;
  let mut refresh = false;

  loop {
    let mut last_key = None;
    while event::poll(Duration::from_millis(0)).unwrap() {
      if let Event::Key(key) = event::read().unwrap() {
        last_key = Some(key);
      }
    }

    if let Some(key) = last_key {
      refresh = true;
      if !command_mode {
        match key.code {
          KeyCode::Char('q') => break,
          KeyCode::Char('w') => app.fractal.z.im -= 0.1 / app.fractal.scale,
          KeyCode::Char('s') => app.fractal.z.im += 0.1 / app.fractal.scale,
          KeyCode::Char('a') => app.fractal.z.re -= 0.1 / app.fractal.scale,
          KeyCode::Char('d') => app.fractal.z.re += 0.1 / app.fractal.scale,
          KeyCode::Char('=') => app.fractal.scale *= 1.1,
          KeyCode::Char('-') => app.fractal.scale /= 1.1,
          KeyCode::Char('r') => app.fractal.max_iterations += 1,
          KeyCode::Char('f') => app.fractal.max_iterations -= 1,
          KeyCode::Char(':') => {
            command_mode = true;
            command_string.clear();
          }
          _ => refresh = false,
        }
      } else {
        match key.code {
          KeyCode::Esc => {
            command_mode = false;
            command_string.clear();
          }
          KeyCode::Enter => {
            let cmd = parse_command(&command_string);
            let _ = execute_command(&mut app, cmd).unwrap_or_else(|err| format!("Error: {}", err));
            command_mode = false;
            command_string.clear();
          }
          KeyCode::Backspace => {
            command_string.pop();
          }
          KeyCode::Char(c) => {
            command_string.push(c);
          }
          _ => {}
        }
      }
    }

    if first_render || refresh {
      first_render = false;
      refresh = false;
      let img = app.fractal.render_frame(width, height, false);
      let mut out = String::new();
      let palette = &app.fractal.palette[app.fractal.current_palette];
      let black_index = palette.stops.len();
      let total_colors = black_index + 1;

      out.push_str("\x1b[H");
      out.push_str("\x1bP9;1q");
      out.push_str(&format!("\"1;1;{};{}", width, height));
      for (i, (r, g, b)) in palette.stops.iter().enumerate() {
        out.push_str(&format!(
          "#{};2;{};{};{}",
          i,
          to_sixel(*r),
          to_sixel(*g),
          to_sixel(*b)
        ));
      }
      out.push_str(&format!("#{};2;0;0;0", black_index));

      for y in (0..height).step_by(6) {
        for c in 0..total_colors {
          out.push_str(&format!("#{}", c));

          for x in 0..width {
            let mut bits = 0;
            for bit in 0..6 {
              if y + bit >= height {
                continue;
              }

              let px = img[(y + bit) as usize][x as usize];
              let matches = if c == black_index {
                px == Rgb([0, 0, 0])
              } else {
                let (r, g, b) = palette.stops[c];
                px == Rgb([r, g, b])
              };

              if matches {
                bits |= 1 << bit;
              }
            }
            out.push((63 + bits) as u8 as char);
          }
          out.push('$');
        }
        out.push('-');
      }
      out.push_str("\x1b\\");
      out.push_str("use wasd to move | =/- to zoom | r/f to change max iterations\n");
      out.push_str(": to enter command mode | q to quit\n");
      if command_mode {
        out.push_str(&format!("command mode: {}\n", command_string));
      } else {
        out.push_str("\x1b[K\n");
      }

      print!("{}", out);
      io::stdout().flush().unwrap();
    }
  }
  disable_raw_mode().unwrap();
}
