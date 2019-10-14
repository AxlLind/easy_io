/*
  A Fast and dead-simple writer for competitive programming in Rust

  Author: Axel Lindeberg, github.com/AxlLind
  Website: https://github.com/AxlLind/EasyIO.rs
  License: MIT
  2019
*/
#![allow(dead_code)]
use std::io::{self, Write};
use std::fs::File;

pub struct OutputWriter {
  writer: Box<dyn Write>,
  buf: Vec<u8>,
}

impl OutputWriter {
  pub fn new() -> Self {
    Self::from_writer(Box::new( io::stdout() ))
  }

  pub fn from_file(path: &str) -> Self {
    let file = Box::new( File::open(path).unwrap() );
    Self::from_writer(file)
  }

  pub fn from_writer(writer: Box<dyn Write>) -> Self {
    Self {
      writer,
      buf: Vec::with_capacity(1 << 16),
    }
  }

  pub fn write(&mut self, s: &str) { self.buf.extend(s.as_bytes()); }

  pub fn writeln(&mut self, s: &str) {
    self.write(s);
    self.buf.push(b'\n');
  }

  pub fn flush(&mut self) {
    self.writer.write_all(&self.buf).unwrap();
    self.buf.clear();
  }
}

impl Drop for OutputWriter {
  fn drop(&mut self) { self.flush(); }
}
