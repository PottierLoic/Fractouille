mod fractal;
mod fractal_colorizer;
mod palettes;
mod utils;

use crate::fractal::{Fractal, Set};
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
  fractal: Fractal,
  show_extended_menu: bool,
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
      term.draw(|f| f.render_widget(&mut self, f.area()))?;
      self.handle_input()?;
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
          KeyCode::Char('h') | KeyCode::Char('H') => {
            self.show_extended_menu = !self.show_extended_menu;
            f.need_render = true;
          }
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
          KeyCode::Char('o') => f.power -= 1.0,
          KeyCode::Char('O') => f.power -= 0.01,
          KeyCode::Char('p') => f.power += 1.0,
          KeyCode::Char('P') => f.power += 0.01,
          KeyCode::Enter => {
            f.set = match f.set {
              Set::Mandelbrot => Set::Julia,
              Set::Julia => Set::BurningShip,
              Set::BurningShip => Set::Mandelbrot,
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
    let layout = if self.show_extended_menu {
      Layout::vertical([Length(7), Min(0)])
    } else {
      Layout::vertical([Length(1), Min(0)])
    };

    let [menu, main] = layout.areas(area);

    if self.show_extended_menu {
      let extended_info = vec![
        format!(
          "Fractouille // Set: {:?} | Palette: {} | Zoom: {:.2}x | Iter: {}",
          self.fractal.set,
          self.fractal.current_palette,
          self.fractal.scale,
          self.fractal.max_iterations
        ),
        "press G to take a high quality screenshot !".to_string(),
        "WASD/Arrows - Move around | +/- - Zoom in/out".to_string(),
        format!(
          "Current position: ({:.6}, {:.6})",
          self.fractal.center_x, self.fractal.center_y
        ),
        format!(
          "Enter - Switch set ({:?}) | R/F - Iter ({}) | Space - Next palette",
          self.fractal.set, self.fractal.max_iterations
        ),
        format!(
          "O/P - decrease / increase power - Shift+O/P for decimals ({:.2}) |",
          self.fractal.power
        ),
        "H - Close extended menu | Q - Quit".to_string(),
      ];

      Text::from(extended_info.join("\n")).render(menu, buf);
    } else {
      let [title, _] = Layout::horizontal([Min(0), Length(8)]).areas(menu);
      Text::from(format!(
        "Fractouille // Set: {:?} | Palette: {} | Zoom: {:.2}x | Iter: {} (H to extend menu)",
        self.fractal.set,
        self.fractal.current_palette,
        self.fractal.scale,
        self.fractal.max_iterations
      ))
      .centered()
      .render(title, buf);
    }

    self.fractal.render(main, buf);
  }
}
