#![allow(dead_code)]
use std::io::{self, Write, Result};
use std::fs::File;

macro_rules! io_error {
  ($s:expr) => { Err(io::Error::new(io::ErrorKind::Other, format!("OutputWriter: {}", $s))) }
}

pub struct OutputWriter {
  writer: Box<dyn Write>,
  buf: Vec<u8>,
}

impl OutputWriter {
  pub fn new() -> OutputWriter {
    OutputWriter::from_writer(Box::new( io::stdout() ))
  }

  pub fn from_file(path: &str) -> Result<OutputWriter> {
    let file = Box::new( File::open(path)? );
    Ok(OutputWriter::from_writer(file))
  }

  pub fn from_writer(writer: Box<dyn Write>) -> OutputWriter {
    OutputWriter {
      writer,
      buf: Vec::with_capacity(1 << 16),
    }
  }

  pub fn write(&mut self, s: &str) { self.buf.extend(s.as_bytes()); }

  pub fn writeln(&mut self, s: &str) {
    self.write(s);
    self.buf.push('\n' as u8);
  }

  pub fn flush(&mut self) -> Result<()> {
    self.writer.write(&self.buf)?;
    self.buf.clear();
    Ok(())
  }

  pub fn set_buf_size(&mut self, buf_size: usize) -> Result<()> {
    if buf_size < self.buf.len() {
      return io_error!("Data loss while shrinking buffer!");
    }
    let additional = buf_size - self.buf.len();
    self.buf.reserve(additional);
    Ok(())
  }
}

impl Drop for OutputWriter {
  fn drop(&mut self) { self.flush().unwrap(); }
}
