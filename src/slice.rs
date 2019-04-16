use std::io;
use json::object::{Object};
use crate::generator::codegen::{Generator, extend_from_slice};

enum WriteSlice {
    Remainder(Vec<u8>)
}

pub struct SliceGenerator {
    code: Vec<WriteSlice>,
}

impl SliceGenerator {
    pub fn new(capacity: usize) -> Self {
        let mut gen = SliceGenerator {
            code: Vec::with_capacity(capacity),
        };
        gen.code.push(WriteSlice::Remainder(Vec::with_capacity(1024)));
        gen
    }

    pub fn consume(&mut self) -> String {
        // Original strings were unicode, numbers are all ASCII,
        // therefore this is safe.
        if let Some(WriteSlice::Remainder(code)) = self.code.pop() {
            unsafe { String::from_utf8_unchecked(code) }
        } else {
            panic!("oh no")
        }
    }
}

impl Generator for SliceGenerator {
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
        if let Some(WriteSlice::Remainder(code)) = self.code.last_mut() {
            code
        } else {
            panic!("oh no")
        }
    }

    #[inline(always)]
    fn write_min(&mut self, _: &[u8], min: u8) -> io::Result<()> {
        self.get_writer().push(min);
        Ok(())
    }
}