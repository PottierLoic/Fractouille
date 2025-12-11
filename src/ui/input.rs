use crate::app::{App, AppState, ProgressEvent};
use crate::command::{execute_command, find_command_autocompletion, parse_command};
use crate::fractal::Set;
use ratatui::crossterm::event;
use ratatui::crossterm::event::{Event, KeyCode, KeyEventKind};
use std::time::Duration;

impl App {
  pub fn handle_input(&mut self) -> color_eyre::Result<()> {
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
          self.fractal_view.need_render = true;

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
        if self.fractal_view.need_render {
          self.fractal_view.colors.clear();
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
