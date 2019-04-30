use std::io;
use std::ptr;
use json::JsonValue;

use crate::generator::codegen::{Generator, extend_from_slice};

#[derive(Debug)]
enum WriteSlice {
    Remainder(Vec<u8>),
    Match(Vec<u8>)
}

impl PartialEq for WriteSlice {
    fn eq(&self, other: &WriteSlice) -> bool {
        match (self, other) {
          (WriteSlice::Remainder(ref left), WriteSlice::Remainder(ref right)) => right == left,
          (WriteSlice::Match(ref left), WriteSlice::Match(ref right)) => right == left,
          (_, _) => false
        }
    }
}

pub struct HighlightGenerator<'a> {
    code: Vec<WriteSlice>,
    slices: Vec<&'a JsonValue>
}

impl<'a> HighlightGenerator<'a> {
    pub fn new() -> Self {
        HighlightGenerator {
            code: vec![],
            slices: vec![]
        }
    }

    pub fn consume(&mut self) -> String {
        // Original strings were unicode, numbers are all ASCII,
        // therefore this is safe.
        match &self.code.last_mut() {
          Some(WriteSlice::Remainder(code)) | Some(WriteSlice::Match(code)) => unsafe {
            String::from_utf8_unchecked(code.to_vec())
          },
          None => String::from("")
        }
    }

    pub fn write_json_with_highlight(&mut self, json: &JsonValue, slices: &mut Vec<&'a JsonValue>) -> io::Result<()> {
      self.slices.append(slices);
      self.write_json(json)
    }

    fn segment(&mut self) {
      self.code.push(WriteSlice::Remainder(Vec::with_capacity(1024)));
    }

    fn match_segment(&mut self) {
      self.code.push(WriteSlice::Match(Vec::with_capacity(1024)));
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
            Some(WriteSlice::Match(ref mut code)) => code,
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
            self.segment();
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
        .as_bytes().to_vec()
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
        .as_bytes().to_vec()
      )
    );
  }
}