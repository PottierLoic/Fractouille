mod command;
mod complex;
mod fractal;
mod palette;

use crate::command::{
  execute_command, find_command_autocompletion, find_command_match, parse_command,
};
use crate::fractal::{Fractal, Set};
use color_eyre::Result;
use ratatui::layout::Direction;
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Gauge};
use ratatui::{
  DefaultTerminal,
  buffer::Buffer,
  crossterm::event::{self, Event, KeyCode, KeyEventKind},
  layout::{Constraint::*, Layout, Rect},
  text::Text,
  widgets::Widget,
};
use std::time::Duration;

pub enum ProgressEvent {
  Progress(f64),
  Finished,
}

#[derive(Debug, Default)]
struct App {
  state: AppState,
  fractal: Fractal,
  show_extended_menu: bool,
  command_mode: bool,
  command_string: String,
  quit_requested: bool,
  command_result: String,
  show_record_popup: bool,
  record_progress: f64,
  progress_rx: Option<std::sync::mpsc::Receiver<ProgressEvent>>,
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
              self.command_result = execute_command(self, parse_command(&*self.command_string))
                .unwrap_or_else(|err| format!("Error: {}", err));
              self.command_string.clear();
            }
            KeyCode::Char(c) => {
              self.command_string.push(c);
            }
            KeyCode::Tab => {
              let full_command = find_command_autocompletion(self.command_string.as_str());
              if let Some(completion) = full_command {
                self.command_string = completion.parse()?;
              }
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
            KeyCode::Char('a') | KeyCode::Left => f.z.re -= step,
            KeyCode::Char('d') | KeyCode::Right => f.z.re += step,
            KeyCode::Char('w') | KeyCode::Up => f.z.im -= step,
            KeyCode::Char('s') | KeyCode::Down => f.z.im += step,
            KeyCode::Char(' ') => f.current_palette = (f.current_palette + 1) % f.palette.len(),
            KeyCode::Enter => {
              f.set = match f.set {
                Set::Mandelbrot => Set::Julia,
                Set::Julia => Set::BurningShip,
                Set::BurningShip => Set::Phoenix,
                Set::Phoenix => Set::Mandelbrot,
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
    let mut finished = false;
    if let Some(rx) = &self.progress_rx {
      while let Ok(event) = rx.try_recv() {
        match event {
          ProgressEvent::Progress(p) => {
            self.record_progress = p;
          }
          ProgressEvent::Finished => {
            finished = true;
          }
        }
      }
    }
    if finished {
      self.progress_rx = None;
      self.show_record_popup = false;
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
          self.fractal.z.re,
          self.fractal.z.im,
          self.fractal.scale,
          self.fractal.max_iterations,
          self.fractal.power
        ),
        if self.fractal.set == Set::Julia {
          format!(
            "C: {:.6} + {:.6}",
            self.fractal.julia_c.re, self.fractal.julia_c.im
          )
        } else if self.fractal.set == Set::Phoenix {
          format!(
            "C: {:.6} + {:.6} | P: {:.6} + {:.6}",
            self.fractal.phoenix_c.re,
            self.fractal.phoenix_c.im,
            self.fractal.phoenix_p.re,
            self.fractal.phoenix_p.im
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
      let mut completion_hint = Some("");
      if !self.command_string.is_empty() {
        completion_hint = find_command_match(&self.command_string);
      }
      Text::from(Line::from(vec![
        Span::styled(
          self.command_string.clone(),
          Style::default().add_modifier(Modifier::BOLD),
        ),
        Span::from(completion_hint.unwrap_or("")),
      ]))
      .render(cmd_bar, buf);
    } else {
      Text::from(self.command_result.to_string()).render(cmd_bar, buf);
    }

    if self.show_record_popup {
      draw_record_popup(area, buf, self);
    }
  }
}

fn draw_record_popup(area: Rect, buf: &mut Buffer, app: &App) {
  let popup_width = 30;
  let popup_height = 5;

  let popup_area = Rect {
    x: area.x + area.width.saturating_sub(popup_width),
    y: area.y,
    width: popup_width,
    height: popup_height,
  };

  for y in popup_area.y..popup_area.y + popup_area.height {
    for x in popup_area.x..popup_area.x + popup_area.width {
      let cell = &mut buf[(x, y)];
      cell.set_bg(Color::Black);
      cell.set_fg(Color::White);
      cell.set_symbol(" ");
    }
  }

  Block::default()
    .title("Recording…")
    .borders(Borders::ALL)
    .render(popup_area, buf);

  let inner = Layout::default()
    .direction(Direction::Vertical)
    .constraints([Length(1), Length(3)])
    .margin(1)
    .split(popup_area);

  Gauge::default()
    .ratio(app.record_progress)
    .label(format!("{:.0}%", app.record_progress * 100.0))
    .render(inner[1], buf);
}
