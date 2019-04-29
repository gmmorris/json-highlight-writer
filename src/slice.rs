use std::io;
use json::object::{Object};
use json::JsonValue;
use json::number::Number;
use crate::generator::codegen::{Generator, extend_from_slice};
use colored::*;
use std::ptr;

enum WriteSlice {
    Remainder(Vec<u8>),
    Match(Vec<u8>, Color),
}

pub struct SliceGenerator<'a> {
    code: Vec<WriteSlice>,
    colors: Vec<Color>,
    active_colors: Vec<Color>,
    slices: Vec<&'a JsonValue>
}

impl<'a> SliceGenerator<'a> {
    pub fn new(capacity: usize, slices: Vec<&'a JsonValue>) -> Self {
        let mut gen = SliceGenerator {
            code: Vec::with_capacity(capacity),
            active_colors: vec![],
            colors: vec![
                Color::Red,
                Color::Green,
                Color::Yellow,
                Color::Blue,
                Color::Magenta,
                Color::Cyan,
                Color::White,
                Color::BrightBlack,
                Color::BrightRed,
                Color::BrightGreen,
                Color::BrightYellow,
                Color::BrightBlue,
                Color::BrightMagenta,
                Color::BrightCyan,
                Color::BrightWhite
            ],
            slices
        };
        gen.code.push(WriteSlice::Remainder(Vec::with_capacity(1024)));
        gen
    }

    pub fn consume(&mut self) -> String {
        let slices : Vec<String> = self.code.iter()
            .map(|slice| match slice {
                    WriteSlice::Match(code, color) => {
                        // Original strings were unicode, numbers are all ASCII,
                        // therefore this is safe.
                        unsafe { String::from_utf8_unchecked(code.to_vec()).color(*color).to_string() }
                    },
                    WriteSlice::Remainder(code) => {
                        // Original strings were unicode, numbers are all ASCII,
                        // therefore this is safe.
                        unsafe { String::from_utf8_unchecked(code.to_vec()) }
                    }
                }
            )
            .collect();
        slices.join("")
    }
}

impl<'a> Generator for SliceGenerator<'a> {
    type T = Vec<u8>;

    fn write(&mut self, slice: &[u8]) -> io::Result<()> {
        extend_from_slice(&mut self.get_writer(), slice);
        Ok(())
    }

    fn write_json(&mut self, json: &JsonValue) -> io::Result<()> {
        let match_index = self.slices.iter().position(|&slice|ptr::eq(json, slice));
        if let Some(index) = match_index {
            let next_color = self.colors.remove(0);
            self.active_colors.push(next_color);
            self.code.push(WriteSlice::Match(Vec::with_capacity(1024), next_color));
        };
        let res = match *json {
            JsonValue::Null               => self.write(b"null"),
            JsonValue::Short(ref short)   => self.write_string(short.as_str()),
            JsonValue::String(ref string) => self.write_string(string),
            JsonValue::Number(ref number) => self.write_number(number),
            JsonValue::Boolean(true)      => self.write(b"true"),
            JsonValue::Boolean(false)     => self.write(b"false"),
            JsonValue::Array(ref array)   => {
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
            },
            JsonValue::Object(ref object) => {
                self.write_object(object)
            }
        };
        if let Some(index) = match_index {
            self.active_colors.pop();
            match self.active_colors.last() {
                Some(color) => self.code.push(WriteSlice::Match(Vec::with_capacity(1024), *color)),
                None => self.code.push(WriteSlice::Remainder(Vec::with_capacity(1024))),
            };
        };
        res
    }

    #[inline(always)]
    fn write_char(&mut self, ch: u8) -> io::Result<()> {
        self.get_writer().push(ch);
        Ok(())
    }

    #[inline(always)]
    fn get_writer(&mut self) -> &mut Vec<u8> {
        match self.code.last_mut() {
            Some(WriteSlice::Remainder(code)) => code,
            Some(WriteSlice::Match(code, _)) => code,
            _ => panic!("oh no"),
        }
    }

    #[inline(always)]
    fn write_min(&mut self, _: &[u8], min: u8) -> io::Result<()> {
        self.get_writer().push(min);
        Ok(())
    }
}