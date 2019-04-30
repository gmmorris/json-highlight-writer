use colored::*;

pub struct SingleColor {
  color: Color
}

impl SingleColor {
  pub fn new() -> Self {
    SingleColor {
      color: Color::Red
    }
  }
}

pub trait HighlightColor {
  fn get_color(&self) -> Color;
}

impl HighlightColor for SingleColor {
  fn get_color(&self) -> Color {
    self.color
  }
}