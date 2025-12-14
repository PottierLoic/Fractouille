use crate::fractal::{Fractal, Set};
use image::Rgb;
use std::io::{self, Write};
use std::process::exit;
use crate::palette::InterpolationMode;

const HEIGHT: u32 = 600;
const WIDTH: u32 = 600;

fn clear_terminal() {
  print!("\x1b[2J\x1b[H");
  io::stdout().flush().unwrap();
}

fn to_sixel(v: u8) -> u32 {
  v as u32 * 100 / 255
}

pub fn start_sixel_rendering() {
  let mut fractal = Fractal::default();
  let mut set_selected = false;
  fractal.palette[0].interpolation = InterpolationMode::None;

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
        fractal.set = Set::Mandelbrot;
        set_selected = true;
      }
      "2" => {
        fractal.set = Set::Julia;
        set_selected = true;
      }
      "3" => {
        fractal.set = Set::BurningShip;
        set_selected = true;
      }
      _ => continue,
    }
    clear_terminal();
  }

  loop {
    let base_width = 3.5;
    let view_width = base_width / fractal.scale;
    let aspect = HEIGHT as f64 / WIDTH as f64;

    let dx = view_width / 4.0;
    let dy = dx * aspect;

    clear_terminal();
    let img = fractal.render_frame(WIDTH, HEIGHT, false);

    let mut out = String::new();

    out.push_str("\x1bPq");
    out.push_str(&format!("\"1;1;{};{}", WIDTH, HEIGHT));
    for (i, (r, g, b)) in fractal.palette[0].stops.iter().enumerate() {
      out.push_str(&format!(
        "#{};2;{};{};{}",
        i,
        to_sixel(*r),
        to_sixel(*g),
        to_sixel(*b)
      ));
    }

    for y in (0..HEIGHT).step_by(6) {
      for c in 0..fractal.palette[0].stops.len() {
        let color = fractal.palette[0].stops[c];
        out.push_str(&format!("#{}", c));

        for x in 0..WIDTH {
          let mut bits = 0;
          for bit in 0..6 {
            if y + bit < HEIGHT
              && img[(y + bit) as usize][x as usize] == Rgb([color.0, color.1, color.2])
            {
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
    print!("{}", out);
    println!(" 1 | 2 ");
    println!("---+---");
    println!(" 3 | 4 ");
    println!("Selection a quadrant to zoom in.");
    println!("type q to quit.");
    io::stdout().flush().unwrap();
    let mut input = String::new();
    if io::stdin().read_line(&mut input).is_err() {
      continue;
    }
    match input.trim() {
      "1" => {
        fractal.z.re -= dx;
        fractal.z.im -= dy;
        fractal.scale *= 2.0;
      }
      "2" => {
        fractal.z.re += dx;
        fractal.z.im -= dy;
        fractal.scale *= 2.0;
      }
      "3" => {
        fractal.z.re -= dx;
        fractal.z.im += dy;
        fractal.scale *= 2.0;
      }
      "4" => {
        fractal.z.re += dx;
        fractal.z.im += dy;
        fractal.scale *= 2.0;
      }
      "q" => exit(0),
      _ => continue,
    }
  }
}
