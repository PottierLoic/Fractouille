use ratatui::prelude::Color;

pub(crate) fn color_to_rgb(color: &Color) -> (u8, u8, u8) {
  match color {
    Color::Black => (0, 0, 0),
    Color::Red => (255, 0, 0),
    Color::Green => (0, 255, 0),
    Color::Yellow => (255, 255, 0),
    Color::Blue => (0, 0, 255),
    Color::Magenta => (255, 0, 255),
    Color::Cyan => (0, 255, 255),
    Color::Gray => (128, 128, 128),
    Color::DarkGray => (64, 64, 64),
    Color::LightRed => (255, 128, 128),
    Color::LightGreen => (128, 255, 128),
    Color::LightYellow => (255, 255, 128),
    Color::LightBlue => (128, 128, 255),
    Color::LightMagenta => (255, 128, 255),
    Color::LightCyan => (128, 255, 255),
    Color::White => (255, 255, 255),
    Color::Rgb(r, g, b) => (*r, *g, *b),
    _ => (0, 0, 0),
  }
}
