use crate::fractal::Fractal;
use crate::ui::FractalView;
use ratatui::DefaultTerminal;

pub enum ProgressEvent {
  Progress(f64),
  Finished,
}

#[derive(Debug, Default, PartialEq, Eq)]
pub enum AppState {
  #[default]
  Running,
  Quit,
}

#[derive(Debug, Default)]
pub struct App {
  pub state: AppState,
  pub fractal: Fractal,
  pub fractal_view: FractalView,
  pub show_extended_menu: bool,
  pub command_mode: bool,
  pub command_string: String,
  pub quit_requested: bool,
  pub command_result: String,
  pub show_record_popup: bool,
  pub record_progress: f64,
  pub progress_rx: Option<std::sync::mpsc::Receiver<ProgressEvent>>,
}

impl App {
  pub fn run(mut self, mut term: DefaultTerminal) -> color_eyre::Result<()> {
    while self.state == AppState::Running {
      term.draw(|f| f.render_widget(&mut self, f.area()))?;
      self.handle_input()?;
    }
    Ok(())
  }
}
