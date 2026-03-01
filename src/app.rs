use crate::fractal::Fractal;
use crate::fractal::Set;
use crate::ui::FractalView;
use ratatui::DefaultTerminal;
use std::time::{Duration, Instant};

const DEEP_ZOOM_POINTS: [(f64, f64); 4] = [
  (-0.743643887037151, 0.13182590420533),
  (-1.25066945943091, 0.02012460614887),
  (0.360240443437614, -0.641313061064804),
  (-0.101096363845623, 0.956286510809142),
];

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

#[derive(Debug, Clone)]
pub struct AutoZoomState {
  pub enabled: bool,
  pub fps: f64,
  pub zoom_factor: f64,
  pub scale_ceiling: f64,
  pub cycle_points: bool,
  pub point_idx: usize,
  pub base_scale: f64,
  pub base_iterations: u32,
  pub last_tick: Option<Instant>,
}

impl Default for AutoZoomState {
  fn default() -> Self {
    Self {
      enabled: false,
      fps: 60.0,
      zoom_factor: 1.02,
      scale_ceiling: 1e13,
      cycle_points: true,
      point_idx: 0,
      base_scale: 1.0,
      base_iterations: 100,
      last_tick: None,
    }
  }
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
  pub auto_zoom: AutoZoomState,
}

impl App {
  pub fn configure_auto_zoom(
    &mut self,
    fps: f64,
    zoom_factor: f64,
    scale_ceiling: f64,
    cycle_points: bool,
  ) {
    let nearest_idx = DEEP_ZOOM_POINTS
      .iter()
      .enumerate()
      .min_by(|(_, a), (_, b)| {
        let da = (self.fractal.z.re - a.0).powi(2) + (self.fractal.z.im - a.1).powi(2);
        let db = (self.fractal.z.re - b.0).powi(2) + (self.fractal.z.im - b.1).powi(2);
        da.total_cmp(&db)
      })
      .map(|(idx, _)| idx)
      .unwrap_or(0);

    self.auto_zoom = AutoZoomState {
      enabled: true,
      fps,
      zoom_factor,
      scale_ceiling,
      cycle_points,
      point_idx: nearest_idx,
      base_scale: self.fractal.scale,
      base_iterations: self.fractal.max_iterations,
      last_tick: None,
    };
  }

  pub fn auto_tick_interval(&self) -> Duration {
    if self.auto_zoom.enabled {
      Duration::from_secs_f64((1.0 / self.auto_zoom.fps).max(0.001))
    } else {
      Duration::from_secs_f64(1.0 / 60.0)
    }
  }

  pub fn tick_auto_zoom(&mut self) {
    if !self.auto_zoom.enabled {
      return;
    }

    let now = Instant::now();
    let interval = self.auto_tick_interval();
    if let Some(last) = self.auto_zoom.last_tick {
      if now.duration_since(last) < interval {
        return;
      }
    }
    self.auto_zoom.last_tick = Some(now);

    self.fractal.scale *= self.auto_zoom.zoom_factor;

    if self.fractal.scale > 0.0 && self.auto_zoom.base_scale > 0.0 {
      let octaves = (self.fractal.scale / self.auto_zoom.base_scale)
        .log2()
        .max(0.0);
      let adaptive = self.auto_zoom.base_iterations + (octaves * 24.0) as u32;
      self.fractal.max_iterations = adaptive;
    }

    if self.fractal.scale > self.auto_zoom.scale_ceiling {
      self.fractal.scale = self.auto_zoom.base_scale;
      self.fractal.max_iterations = self.auto_zoom.base_iterations;

      if self.auto_zoom.cycle_points && self.fractal.set == Set::Mandelbrot {
        self.auto_zoom.point_idx = (self.auto_zoom.point_idx + 1) % DEEP_ZOOM_POINTS.len();
        let (re, im) = DEEP_ZOOM_POINTS[self.auto_zoom.point_idx];
        self.fractal.z.re = re;
        self.fractal.z.im = im;
      }
    }

    self.fractal_view.need_render = true;
  }

  pub fn run(mut self, mut term: DefaultTerminal) -> color_eyre::Result<()> {
    while self.state == AppState::Running {
      term.draw(|f| f.render_widget(&mut self, f.area()))?;
      self.handle_input()?;
    }
    Ok(())
  }
}
