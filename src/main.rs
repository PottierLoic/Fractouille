mod fractal;
mod palettes;
mod utils;

use crate::fractal::{FractalWidget, Set};
use color_eyre::Result;
use ratatui::{
  DefaultTerminal,
  buffer::Buffer,
  crossterm::event::{self, Event, KeyCode, KeyEventKind},
  layout::{Constraint::*, Layout, Rect},
  text::Text,
  widgets::Widget,
};
use std::time::Duration;

#[derive(Debug, Default)]
struct App {
  state: AppState,
  fractal: FractalWidget,
  frame_counter: u64,
}

#[derive(Debug, Default, PartialEq, Eq)]
enum AppState {
  #[default]
  Running,
  Quit,
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
      let t = self.frame_counter as f64 * 0.02;
      self.fractal.real = 0.7885 * t.cos();
      self.fractal.imag = 0.7885 * t.sin();
      self.fractal.need_render = true;

      term.draw(|f| f.render_widget(&mut self, f.area()))?;
      self.handle_input()?;
      self.frame_counter += 1;
    }
    Ok(())
  }

  fn handle_input(&mut self) -> Result<()> {
    let timeout = Duration::from_secs_f32(1.0 / 60.0);
    let mut save_requested = false;
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
          KeyCode::Enter => {
            f.set = match f.set {
              Set::Mandelbrot => Set::Julia,
              Set::Julia => Set::Mandelbrot,
            }
          }
          KeyCode::Char('g') => save_requested = true,
          _ => {}
        }
        if f.need_render {
          f.colors.clear();
        }
      }
    }
    if save_requested {
      self.fractal.save_screenshot();
    }
    Ok(())
  }
}

impl Widget for &mut App {
  fn render(self, area: Rect, buf: &mut Buffer) {
    let [top, main] = Layout::vertical([Length(1), Min(0)]).areas(area);
    let [title, _] = Layout::horizontal([Min(0), Length(8)]).areas(top);
    Text::from(
      format!(
        "Fractouille // Set: {:?} | Palette: {} | Zoom: {:.2}x | Iter: {} // +/- zoom | wasd move | r/f iterations | space palettes | enter sets | g save",
        self.fractal.set,
        self.fractal.current_palette,
        self.fractal.scale,
        self.fractal.max_iterations
      )
    ).centered().render(title, buf);
    self.fractal.render(main, buf);
  }
}
