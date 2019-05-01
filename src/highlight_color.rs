use colored::*;

pub trait HighlightColor {
  fn get_color(&mut self) -> Color;
}

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

impl HighlightColor for SingleColor {
  fn get_color(&mut self) -> Color {
    self.color
  }
}

pub struct CycledColors {
  colors: Box<Iterator<Item = Color>>
}

impl CycledColors {
  pub fn new(colors: Vec<Color>) -> Self {
    CycledColors {
      colors: Box::new(colors.into_iter().cycle())
    }
  }
}

impl HighlightColor for CycledColors {
  fn get_color(&mut self) -> Color {
    self.colors.next().unwrap().clone()
  }
}


#[cfg(test)]
mod tests {
  use super::*;
  use json::*;

  #[test]
  fn cycledcolors_should_cycle_through_colors() {
    let mut cycle = CycledColors::new(
      vec![
        Color::Red,
        Color::Green,
        Color::Yellow
      ]
    );
    
    assert_eq!(cycle.get_color(), Color::Red);
    assert_eq!(cycle.get_color(), Color::Green);
    assert_eq!(cycle.get_color(), Color::Yellow);
    assert_eq!(cycle.get_color(), Color::Red);
  }
}