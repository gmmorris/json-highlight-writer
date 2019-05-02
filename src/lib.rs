use json::JsonValue;
use colored::*;

mod generator;
mod highlight_color;
mod highlight;

pub fn highlight(json_object: &JsonValue, mut slices: Vec<&JsonValue>) -> String {
    let mut gen = highlight::HighlightGenerator::new();
    gen.write_json_with_highlight(
      json_object, &mut slices
    ).expect("Can't fail");
    gen.consume()
}

pub fn highlight_with_colors(json_object: &JsonValue, mut slices: Vec<&JsonValue>, colors: Vec<Color>) -> String {
    let mut gen = highlight::HighlightGenerator::new_with_colors(colors);
    gen.write_json_with_highlight(
      json_object, &mut slices
    ).expect("Can't fail");
    gen.consume()
}

pub fn highlight_with_colors_and_remainder(json_object: &JsonValue, mut slices: Vec<&JsonValue>, colors: Option<Vec<Color>>, remainder_color: Option<Color>) -> String {
    let mut gen = highlight::HighlightGenerator::new_with_colors_and_remainder(colors, remainder_color);
    gen.write_json_with_highlight(
      json_object, &mut slices
    ).expect("Can't fail");
    gen.consume()
}
