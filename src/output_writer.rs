/*
  A Fast and dead-simple writer for competitive programming in Rust

  Author: Axel Lindeberg, github.com/AxlLind
  Website: https://github.com/AxlLind/EasyIO.rs
  License: MIT
  2019
*/
#![allow(dead_code)]
use std::io::{self, Write, Result};
use std::fs::File;

pub struct OutputWriter {
  writer: Box<dyn Write>,
  buf: Vec<u8>,
}

// Constructors
impl OutputWriter {
  pub fn new() -> Self {
    let stdout = Box::new( io::stdout() );
    Self::from_writer(stdout)
  }

  pub fn from_file(path: &str) -> Self {
    let file = Box::new( File::open(path).unwrap() );
    Self::from_writer(file)
  }

  pub fn from_writer(writer: Box<dyn Write>) -> Self {
    let buf = Vec::with_capacity(1 << 16);
    Self { writer, buf }
  }
}

// Instance methods
impl OutputWriter {
  pub fn print(&mut self, s: &str) {
    self.buf.extend(s.as_bytes());
  }

  pub fn println(&mut self, s: &str) {
    self.buf.extend(s.as_bytes());
    self.buf.push(b'\n');
  }
}

impl Write for OutputWriter {
  fn write(&mut self, s: &[u8]) -> Result<usize> {
    self.buf.extend(s);
    Ok(s.len())
  }

  fn flush(&mut self) -> Result<()> {
    self.writer.write_all(&self.buf)?;
    self.buf.clear();
    Ok(())
  }
}

impl Drop for OutputWriter {
  fn drop(&mut self) { self.flush().unwrap(); }
}
