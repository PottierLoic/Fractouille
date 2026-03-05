use crate::fractal::Fractal;
use std::time::{Duration, Instant};

#[derive(Debug, Clone)]
pub struct AnimationState {
  pub enabled: bool,
  pub fps: f64,
  pub speed: f64,
  last_tick: Option<Instant>,
}

impl Default for AnimationState {
  fn default() -> Self {
    Self {
      enabled: false,
      fps: 18.0,
      speed: 1.02,
      last_tick: None,
    }
  }
}

impl AnimationState {
  pub fn tick_interval(&self) -> Duration {
    if self.enabled {
      Duration::from_secs_f64(1.0 / self.fps.max(0.1))
    } else {
      Duration::from_secs_f64(1.0 / 60.0)
    }
  }

  pub fn set_enabled(&mut self, enabled: bool) {
    self.enabled = enabled;
    self.last_tick = None;
  }

  pub fn toggle(&mut self) {
    self.set_enabled(!self.enabled);
  }

  pub fn set_fps(&mut self, fps: f64) -> Result<(), String> {
    if fps <= 0.0 {
      return Err("Animation FPS must be positive".to_string());
    }
    self.fps = fps;
    self.last_tick = None;
    Ok(())
  }

  pub fn set_speed(&mut self, speed: f64) -> Result<(), String> {
    if speed <= 0.0 {
      return Err("Animation speed must be positive".to_string());
    }
    self.speed = speed;
    Ok(())
  }

  pub fn tick(&mut self, fractal: &mut Fractal) -> bool {
    if !self.enabled {
      return false;
    }

    let now = Instant::now();
    let interval = self.tick_interval();

    if let Some(last_tick) = self.last_tick {
      if now.duration_since(last_tick) < interval {
        return false;
      }
    }

    self.last_tick = Some(now);
    fractal.scale *= self.speed;
    true
  }
}
