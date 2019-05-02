use std::io;
use std::ptr;
use json::JsonValue;
use colored::*;

use crate::highlight_color::{HighlightColor, SingleColor, CycledColors};

use crate::generator::codegen::{Generator, extend_from_slice};

#[derive(Debug)]
enum WriteSlice {
    Remainder(Vec<u8>),
    Match(Vec<u8>, Color)
}

impl PartialEq for WriteSlice {
    fn eq(&self, other: &WriteSlice) -> bool {
        match (self, other) {
          (WriteSlice::Remainder(ref left), WriteSlice::Remainder(ref right)) => right == left,
          (WriteSlice::Match(ref left, ref lcolor), WriteSlice::Match(ref right, ref rcolor)) => right == left && rcolor == lcolor,
          (_, _) => false
        }
    }
}

pub struct HighlightGenerator<'a> {
    code: Vec<WriteSlice>,
    slices: Vec<&'a JsonValue>,
    color: Box<HighlightColor>,
    remainder_color: Option<Color>
}

impl<'a> HighlightGenerator<'a> {
    pub fn new() -> Self {
        HighlightGenerator {
            code: vec![],
            slices: vec![],
            color: Box::new(SingleColor::new()),
            remainder_color: None
        }
    }

    pub fn new_with_colors(colors: Option<Vec<Color>>, remainder_color: Option<Color>) -> Self {
        HighlightGenerator {
            code: vec![],
            slices: vec![],
            color: match colors {
              Some(colors) => Box::new(CycledColors::new(colors)),
              None => Box::new(SingleColor::new())
            },
            remainder_color
        }
    }

    pub fn consume(&mut self) -> String {
        let slices : Vec<String> = self.code.iter()
            .map(|slice| match (slice, self.remainder_color) {
                // Original strings were unicode, numbers are all ASCII,
                // therefore this is safe.
                (WriteSlice::Match(code, ref color),_) | (WriteSlice::Remainder(code), Some(ref color)) => {
                    unsafe { String::from_utf8_unchecked(code.to_vec()).color(*color).to_string() }
                },
                (WriteSlice::Remainder(code), None) => {
                    unsafe { String::from_utf8_unchecked(code.to_vec()) }
                }
            })
            .collect();
        slices.join("")
    }

    pub fn write_json_with_highlight(&mut self, json: &JsonValue, slices: &mut Vec<&'a JsonValue>) -> io::Result<()> {
      self.slices.append(slices);
      self.write_json(json)
    }

    fn current_color(&self) -> Option<Color> {
      match self.code.last() {
        Some(WriteSlice::Match(_, ref color)) => Some(color.clone()),
        _ => None
      }
    }

    fn segment(&mut self, color : Option<Color>) {
      self.code.push(
        match color {
          Some(color) => WriteSlice::Match(Vec::with_capacity(1024), color),
          None => WriteSlice::Remainder(Vec::with_capacity(1024))
        }
      );
    }

    fn match_segment(&mut self) {
      let color = self.get_color();
      self.segment(Some(color));
    }

    fn get_color(&mut self) -> Color {
      self.color.get_color()
    }

    fn write_array(&mut self, array: &Vec<JsonValue>) -> io::Result<()> {
        self.write_char(b'[')?;
        let mut iter = array.iter();

        if let Some(item) = iter.next() {
            self.write_json(item)?;
        } else {
            self.write_char(b']')?;
            return Ok(());
        }

        for item in iter {
            self.write_char(b',')?;
            self.write_json(item)?;
        }

        self.write_char(b']')
    }
}

impl<'a> Generator for HighlightGenerator<'a> {
    type T = Vec<u8>;

    fn write(&mut self, slice: &[u8]) -> io::Result<()> {
        extend_from_slice(&mut self.get_writer(), slice);
        Ok(())
    }

    #[inline(always)]
    fn write_char(&mut self, ch: u8) -> io::Result<()> {
        self.get_writer().push(ch);
        Ok(())
    }

    #[inline(always)]
    fn get_writer(&mut self) -> &mut Vec<u8> {
        if self.code.is_empty() {
          self.code.push(WriteSlice::Remainder(Vec::with_capacity(1024)));
        };
        match self.code.last_mut() {
            Some(WriteSlice::Remainder(ref mut code)) => code,
            Some(WriteSlice::Match(ref mut code, _)) => code,
            None => panic!("Internal Error: No writer")
        }
    }

    #[inline(always)]
    fn write_min(&mut self, _: &[u8], min: u8) -> io::Result<()> {
        self.get_writer().push(min);
        Ok(())
    }

    fn write_json(&mut self, json: &JsonValue) -> io::Result<()> {
        let match_index = self.slices.iter().position(|&slice|ptr::eq(json, slice));
        let current_color = self.current_color();
        if let Some(_) = match_index {
            self.match_segment();
        };

        let inner_io = match *json {
            JsonValue::Null               => self.write(b"null"),
            JsonValue::Short(ref short)   => self.write_string(short.as_str()),
            JsonValue::String(ref string) => self.write_string(string),
            JsonValue::Number(ref number) => self.write_number(number),
            JsonValue::Boolean(true)      => self.write(b"true"),
            JsonValue::Boolean(false)     => self.write(b"false"),
            JsonValue::Array(ref array)   => {
                self.write_array(array)
            },
            JsonValue::Object(ref object) => {
                self.write_object(object)
            }
        };
        
        if let Some(_) = match_index {
            self.segment(current_color);
        };
        inner_io
    }
}

#[cfg(test)]
mod tests {
  use super::*;
  use json::*;

  #[test]
  fn should_implement_basic_json_dump() {
    let input = array![json::Null, "world", true];
    let mut gen = HighlightGenerator::new();
    gen.write_json(
      &input
    ).expect("Can't fail");
    assert_eq!(gen.consume(), r#"[null,"world",true]"#);
  }

  #[test]
  fn should_store_slices_to_highlight() {
    let input = object!{
      "foo" => false,
      "bar" => json::Null,
      "answer" => 42,
      "list" => array![json::Null, "world", true]
    };

    let mut slices = vec![
      &input["bar"],
      &input["list"]
    ];

    let mut gen = HighlightGenerator::new();

    gen.write_json_with_highlight(
      &input, &mut slices
    ).expect("Can't fail");

    assert!(ptr::eq(gen.slices[0], &input["bar"]));
    assert!(ptr::eq(gen.slices[1], &input["list"]));
  }

  #[test]
  fn should_segment_slices() {
    let input = object!{
      "foo" => false,
      "bar" => json::Null,
      "answer" => 42,
      "list" => array![json::Null, "world", true]
    };
    let mut slices = vec![
      &input["list"]
    ];

    let mut gen = HighlightGenerator::new();

    gen.write_json_with_highlight(
      &input, &mut slices
    ).expect("Can't fail");

    assert_eq!(
      gen.code[0],
      WriteSlice::Remainder(
        "{\"foo\":false,\"bar\":null,\"answer\":42,\"list\":"
        .as_bytes().to_vec()
      )
    );
    assert_eq!(
      gen.code[1],
      WriteSlice::Match(
        "[null,\"world\",true]"
        .as_bytes().to_vec(),
        Color::Red
      )
    );
    assert_eq!(
      gen.code[2],
      WriteSlice::Remainder(
        "}"
        .as_bytes().to_vec()
      )
    );
  }

  #[test]
  fn should_segment_a_root_slice() {
    let input = object!{
      "foo" => false,
      "bar" => json::Null,
      "answer" => 42,
      "list" => array![json::Null, "world", true]
    };
    let mut slices = vec![
      &input
    ];

    let mut gen = HighlightGenerator::new();

    gen.write_json_with_highlight(
      &input, &mut slices
    ).expect("Can't fail");

    assert_eq!(
      gen.code[0],
      WriteSlice::Match(
        "{\"foo\":false,\"bar\":null,\"answer\":42,\"list\":[null,\"world\",true]}"
        .as_bytes().to_vec(),
        Color::Red
      )
    );
  }

  #[test]
  fn should_highlight_a_single_match() {
      let input = object!{
        "foo" => false,
        "bar" => json::Null,
        "answer" => 42,
        "list" => array![json::Null, "world", true]
      };

      let mut slices = vec![
        &input["list"]
      ];

      let mut gen = HighlightGenerator::new();

      gen.write_json_with_highlight(
        &input, &mut slices
      ).expect("Can't fail");

    assert_eq!(
      gen.consume(),
      format!(
        "{}{}{}",
        r#"{"foo":false,"bar":null,"answer":42,"list":"#,
        r#"[null,"world",true]"#.red(),
        r#"}"#
      )      
    );
  }

  #[test]
  fn should_highlight_remainder() {
      let input = object!{
        "foo" => false,
        "bar" => json::Null,
        "answer" => 42,
        "list" => array![json::Null, "world", true]
      };

      let mut slices = vec![
        &input["list"]
      ];

      let mut gen = HighlightGenerator::new_with_colors(None, Some(Color::White));

      gen.write_json_with_highlight(
        &input, &mut slices
      ).expect("Can't fail");

    assert_eq!(
      gen.consume(),
      format!(
        "{}{}{}",
        r#"{"foo":false,"bar":null,"answer":42,"list":"#.white(),
        r#"[null,"world",true]"#.red(),
        r#"}"#.white()
      )
    );
  }

  #[test]
  fn should_highlight_multiple_matchs() {
      let input = object!{
        "foo" => false,
        "bar" => json::Null,
        "answer" => 42,
        "list" => array![json::Null, "world", true]
      };

      let mut slices = vec![
        &input["bar"],
        &input["list"]
      ];

      let mut gen = HighlightGenerator::new();

      gen.write_json_with_highlight(
        &input, &mut slices
      ).expect("Can't fail");

    assert_eq!(
      gen.consume(),
      format!(
        "{}{}{}{}{}",
        r#"{"foo":false,"bar":"#,
        r#"null"#.red(),
        r#","answer":42,"list":"#,
        r#"[null,"world",true]"#.red(),
        r#"}"#
      )      
    );
  }

  #[test]
  fn should_highlight_multiple_matchs_with_cycled_colors() {
      let input = object!{
        "foo" => false,
        "bar" => json::Null,
        "answer" => 42,
        "list" => array![json::Null, "world", true]
      };

      let mut slices = vec![
        &input["bar"],
        &input["list"]
      ];

      let mut gen = HighlightGenerator::new_with_colors(Some(vec![Color::Red, Color::Green]), None);

      gen.write_json_with_highlight(
        &input, &mut slices
      ).expect("Can't fail");

    assert_eq!(
      gen.consume(),
      format!(
        "{}{}{}{}{}",
        r#"{"foo":false,"bar":"#,
        r#"null"#.red(),
        r#","answer":42,"list":"#,
        r#"[null,"world",true]"#.green(),
        r#"}"#
      )      
    );
  }

  #[test]
  fn should_highlight_inner_matches() {
      let input = object!{
        "foo" => false,
        "bar" => json::Null,
        "answer" => 42,
        "list" => array![json::Null, "world", true]
      };

      let mut slices = vec![
        &input,
        &input["list"]
      ];

      let mut gen = HighlightGenerator::new_with_colors(Some(vec![Color::Red, Color::Green]), None);

      gen.write_json_with_highlight(
        &input, &mut slices
      ).expect("Can't fail");

    assert_eq!(
      gen.consume(),
      format!(
        "{}{}{}",
        r#"{"foo":false,"bar":null,"answer":42,"list":"#.red(),
        r#"[null,"world",true]"#.green(),
        r#"}"#.red()
      )      
    );
  }
}