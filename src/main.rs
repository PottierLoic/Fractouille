use color_eyre::Result;
use ratatui::{
  DefaultTerminal,
  buffer::Buffer,
  crossterm::event::{self, Event, KeyCode, KeyEventKind},
  layout::{Constraint::*, Layout, Position, Rect},
  style::Color,
  text::Text,
  widgets::Widget,
};
use std::time::Duration;

type PaletteFn = fn(t: f64) -> Color;

#[derive(Debug, Default)]
struct App {
  state: AppState,
  fractal: FractalWidget,
}

#[derive(Debug, Default, PartialEq, Eq)]
enum AppState {
  #[default]
  Running,
  Quit,
}

#[derive(Debug)]
struct FractalWidget {
  colors: Vec<Vec<Color>>,
  center_x: f64,
  center_y: f64,
  scale: f64,
  max_iterations: u32,
  need_render: bool,
  palettes: Vec<PaletteFn>,
  current_palette: usize,
}

impl Default for FractalWidget {
  fn default() -> Self {
    fn default_palette(t: f64) -> Color {
      let (r, g, b) = (
        (9.0 * (1.0 - t) * t * t * t * 255.0) as u8,
        (15.0 * (1.0 - t).powi(2) * t * t * 255.0) as u8,
        (8.5 * (1.0 - t).powi(3) * t * 255.0) as u8,
      );
      Color::Rgb(r, g, b)
    }

    fn fire_palette(t: f64) -> Color {
      let r = (255.0 * t) as u8;
      let g = (255.0 * t.powf(0.5) * (1.0 - t)) as u8;
      let b = (64.0 * (1.0 - t)) as u8;
      Color::Rgb(r, g, b)
    }

    fn rainbow_palette(t: f64) -> Color {
      let r = (127.5 * (1.0 + (6.0 * t).sin())) as u8;
      let g = (127.5 * (1.0 + (6.0 * t + 2.0).sin())) as u8;
      let b = (127.5 * (1.0 + (6.0 * t + 4.0).sin())) as u8;
      Color::Rgb(r, g, b)
    }

    fn ocean_palette(t: f64) -> Color {
      let r = (20.0 * (1.0 - t)) as u8;
      let g = (80.0 + 120.0 * t) as u8;
      let b = (200.0 + 55.0 * t) as u8;
      Color::Rgb(r, g, b)
    }

    fn grayscale_palette(t: f64) -> Color {
      let shade = (255.0 * t) as u8;
      Color::Rgb(shade, shade, shade)
    }

    fn electric_palette(t: f64) -> Color {
      let r = (100.0 * (1.0 - t)) as u8;
      let g = (200.0 * t) as u8;
      let b = (255.0 * (t * 1.2).min(1.0)) as u8;
      Color::Rgb(r, g, b)
    }

    Self {
      colors: vec![],
      center_x: -0.5,
      center_y: 0.0,
      scale: 1.0,
      max_iterations: 100,
      need_render: true,
      palettes: vec![
        default_palette,
        fire_palette,
        rainbow_palette,
        ocean_palette,
        grayscale_palette,
        electric_palette,
      ],
      current_palette: 0,
    }
  }
}

fn main() -> Result<()> {
  color_eyre::install()?;
  let term = ratatui::init();
  let res = App::default().run(term);
  ratatui::restore();
  res
}

impl App {
  fn run(mut self, mut term: DefaultTerminal) -> Result<()> {
    while self.state == AppState::Running {
      term.draw(|f| f.render_widget(&mut self, f.area()))?;
      self.handle_input()?;
    }
    Ok(())
  }

  fn handle_input(&mut self) -> Result<()> {
    let timeout = Duration::from_secs_f32(1.0 / 60.0);
    if event::poll(timeout)? {
      if let Event::Key(key) = event::read()? {
        if key.kind != KeyEventKind::Press {
          return Ok(());
        }
        let f = &mut self.fractal;
        let step = 0.1 / f.scale;
        f.need_render = true;

        match key.code {
          KeyCode::Char('q') => self.state = AppState::Quit,
          KeyCode::Char('+') | KeyCode::Char('=') => f.scale *= 1.1,
          KeyCode::Char('-') => f.scale /= 1.1,
          KeyCode::Char('r') => f.max_iterations += 1,
          KeyCode::Char('f') => f.max_iterations = f.max_iterations.saturating_sub(1),
          KeyCode::Char('a') | KeyCode::Left => f.center_x -= step,
          KeyCode::Char('d') | KeyCode::Right => f.center_x += step,
          KeyCode::Char('w') | KeyCode::Up => f.center_y -= step,
          KeyCode::Char('s') | KeyCode::Down => f.center_y += step,
          KeyCode::Char(' ') => f.current_palette = (f.current_palette + 1) % f.palettes.len(),
          _ => f.need_render = false,
        }
        if f.need_render {
          f.colors.clear();
        }
      }
    }
    Ok(())
  }
}

impl Widget for &mut App {
  fn render(self, area: Rect, buf: &mut Buffer) {
    let [top, main] = Layout::vertical([Length(1), Min(0)]).areas(area);
    let [title, _] = Layout::horizontal([Min(0), Length(8)]).areas(top);
    Text::from("Fractouille // +/- zoom | wasd move | r/f iterations | space change palette")
      .centered()
      .render(title, buf);
    self.fractal.render(main, buf);
  }
}

impl Widget for &mut FractalWidget {
  fn render(self, area: Rect, buf: &mut Buffer) {
    self.compute_mandelbrot(area);
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

impl FractalWidget {
  fn compute_mandelbrot(&mut self, area: Rect) {
    let (w, h) = (area.width as usize, area.height as usize * 2);
    if self.colors.len() == h && self.colors[0].len() == w || !self.need_render {
      return;
    }

    let aspect = w as f64 / h as f64;
    let vw = 3.5 / self.scale;
    let vh = vw / aspect;
    let (left, top) = (self.center_x - vw / 2.0, self.center_y - vh / 2.0);

    self.colors = (0..h)
      .map(|y| {
        (0..w)
          .map(|x| {
            let cx = left + x as f64 * vw / w as f64;
            let cy = top + y as f64 * vh / h as f64;
            let mut zx = 0.0;
            let mut zy = 0.0;
            let mut i = 0;

            while zx * zx + zy * zy <= 4.0 && i < self.max_iterations {
              let tmp = zx * zx - zy * zy + cx;
              zy = 2.0 * zx * zy + cy;
              zx = tmp;
              i += 1;
            }

            if i == self.max_iterations {
              Color::Black
            } else {
              let t = i as f64 / self.max_iterations as f64;
              (self.palettes[self.current_palette])(t)
            }
          })
          .collect()
      })
      .collect();
  }
}
