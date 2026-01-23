use crate::fractal::Fractal;
use ratatui::{
  buffer::Buffer,
  layout::{Position, Rect},
  prelude::Color,
};

#[derive(Debug)]
pub struct FractalView {
  pub colors: Vec<Vec<Color>>,
  pub need_render: bool,
}

impl Default for FractalView {
  fn default() -> Self {
    Self {
      colors: vec![],
      need_render: true,
    }
  }
}

impl FractalView {
  pub fn compute(&mut self, fractal: &Fractal, area: Rect) {
    let (w, h) = (area.width as usize, area.height as usize * 2);

    if self.colors.len() == h && self.colors[0].len() == w && !self.need_render {
      return;
    }

    let raw = fractal.render_frame(w as u32, h as u32, false);

    self.colors = raw
      .chunks(w)
      .map(|row| {
        row
          .iter()
          .map(|rgb| Color::Rgb(rgb[0], rgb[1], rgb[2]))
          .collect()
      })
      .collect();

    self.need_render = false;
  }

  pub fn render_fractal(&mut self, fractal: &Fractal, area: Rect, buf: &mut Buffer) {
    self.compute(fractal, area);

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
