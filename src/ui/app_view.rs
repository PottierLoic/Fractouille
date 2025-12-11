use crate::app::App;
use crate::command::find_command_match;
use crate::fractal::Set;
use ratatui::buffer::Buffer;
use ratatui::layout::Constraint::{Length, Min};
use ratatui::layout::{Direction, Layout, Rect};
use ratatui::prelude::{Color, Line, Modifier, Span, Style, Text, Widget};
use ratatui::widgets::{Block, Borders, Gauge};

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

    self.fractal_view.render_fractal(&self.fractal, main, buf);

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
