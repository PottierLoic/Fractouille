mod command;
mod fractal;
mod fractal_maths;
mod palettes;

use crate::command::CommandProcessor;
use crate::fractal::{Fractal, Set};
use crate::palettes::PALETTES;
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
  command_mode: bool,
  command_string: String,
  quit_requested: bool,
  command_result: String,
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
    if event::poll(timeout)? {
      if let Event::Key(key) = event::read()? {
        if key.kind != KeyEventKind::Press {
          return Ok(());
        }

        if self.command_mode {
          match key.code {
            KeyCode::Esc => {
              self.command_mode = false;
              self.command_string.clear();
            }
            KeyCode::Backspace => {
              self.command_string.pop();
            }
            KeyCode::Enter => {
              self.command_mode = false;
              self.command_result =
                CommandProcessor::execute(self).unwrap_or_else(|err| format!("Error: {}", err));
              self.command_string.clear();
            }
            KeyCode::Char(c) => {
              self.command_string.push(c);
            }
            _ => {}
          }
        } else {
          let f = &mut self.fractal;
          let step = 0.1 / f.scale;
          f.need_render = true;

          match key.code {
            KeyCode::Char(':') => {
              self.command_mode = true;
              self.command_string.clear();
            }
            KeyCode::Char('q') => self.quit_requested = true,
            KeyCode::Char('+') | KeyCode::Char('=') => f.scale *= 1.1,
            KeyCode::Char('-') => f.scale /= 1.1,
            KeyCode::Char('r') => f.max_iterations += 1,
            KeyCode::Char('f') => f.max_iterations = f.max_iterations.saturating_sub(1),
            KeyCode::Char('a') | KeyCode::Left => f.z.0 -= step,
            KeyCode::Char('d') | KeyCode::Right => f.z.0 += step,
            KeyCode::Char('w') | KeyCode::Up => f.z.1 -= step,
            KeyCode::Char('s') | KeyCode::Down => f.z.1 += step,
            KeyCode::Char(' ') => f.current_palette = (f.current_palette + 1) % PALETTES.len(),
            KeyCode::Enter => {
              f.set = match f.set {
                Set::Mandelbrot => Set::Julia,
                Set::Julia => Set::BurningShip,
                Set::BurningShip => Set::Mandelbrot,
              }
            }
            _ => {}
          }
        }
        if self.fractal.need_render {
          self.fractal.colors.clear();
        }
      }
    }
    if self.quit_requested {
      self.state = AppState::Quit;
    }
    Ok(())
  }
}

impl Widget for &mut App {
  fn render(self, area: Rect, buf: &mut Buffer) {
    let layout = if self.show_extended_menu {
      Layout::vertical([Length(5), Min(0), Length(1)])
    } else {
      Layout::vertical([Length(1), Min(0), Length(1)])
    };

    let [menu, main, cmd_bar] = layout.areas(area);

    if self.show_extended_menu {
      let extended_info = [
        format!(
          "Fractouille - {:?} | Palette: {}",
          self.fractal.set, self.fractal.current_palette
        ),
        format!(
          "Position: ({:.6}, {:.6}) | Zoom: {:.2}x | Iterations: {} | Power: {:.2}",
          self.fractal.z.0,
          self.fractal.z.1,
          self.fractal.scale,
          self.fractal.max_iterations,
          self.fractal.power
        ),
        if self.fractal.set == Set::Julia {
          format!(
            "Julia Constant: {:.6} + {:.6}",
            self.fractal.julia_constant.0, self.fractal.julia_constant.1
          )
        } else {
          "".to_string()
        },
        "wasd to move | r/f to change max iteration | -/+ to zoom | q to quit".to_string(),
        if self.command_mode {
          "Command Mode: ACTIVE (ESC to exit) | Type 'commands' for command list"
        } else {
          "Press ':' to enter command mode | See COMMANDS.md for available commands"
        }
        .to_string(),
      ];
      Text::from(extended_info.join("\n")).render(menu, buf);
    } else {
      let [title, _] = Layout::horizontal([Min(0), Length(8)]).areas(menu);
      Text::from(format!(
        "Fractouille // Set: {:?} | Palette: {} | Zoom: {:.2}x | Iter: {} (:h to extend menu)",
        self.fractal.set,
        self.fractal.current_palette,
        self.fractal.scale,
        self.fractal.max_iterations
      ))
      .centered()
      .render(title, buf);
    }

    self.fractal.render(main, buf);

    if self.command_mode {
      Text::from(format!(":{}", self.command_string)).render(cmd_bar, buf);
    } else {
      Text::from(self.command_result.to_string()).render(cmd_bar, buf);
    }
  }
}
