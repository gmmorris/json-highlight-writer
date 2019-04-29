use std::io;
use std::ptr;
use json::JsonValue;

use crate::generator::codegen::{Generator, extend_from_slice};

enum WriteSlice {
    Remainder(Vec<u8>)
}

pub struct HighlightGenerator<'a> {
    code: WriteSlice,
    slices: Vec<&'a JsonValue>
}

impl<'a> HighlightGenerator<'a> {
    pub fn new() -> Self {
        HighlightGenerator {
            code: WriteSlice::Remainder(Vec::with_capacity(1024)),
            slices: vec![]
        }
    }

    pub fn consume(&mut self) -> String {
        // Original strings were unicode, numbers are all ASCII,
        // therefore this is safe.
        match &self.code {
          WriteSlice::Remainder(code) => unsafe {
            String::from_utf8_unchecked(code.to_vec())
          }
        }
    }

    pub fn write_json_with_highlight(&mut self, json: &JsonValue, slices: &mut Vec<&'a JsonValue>) -> io::Result<()> {
      self.slices.append(slices);
      self.write_json(json)
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
        match self.code {
            WriteSlice::Remainder(ref mut code) => code
        }
    }

    #[inline(always)]
    fn write_min(&mut self, _: &[u8], min: u8) -> io::Result<()> {
        self.get_writer().push(min);
        Ok(())
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
}