#![allow(dead_code)]
use std::io::{self, Read, Result};
use std::fs::File;

macro_rules! io_error {
  ($s:expr) => { Err(io::Error::new(io::ErrorKind::Other, format!("InputReader: {}", $s))) }
}

pub struct InputReader {
  reader: Box<dyn Read>,
  buf: Vec<u8>,
  bytes_read: usize,
  current_index: usize,
  str_buf: String,
}

// Constructors
impl InputReader {
  pub fn new() -> InputReader {
    InputReader::from_reader(Box::new( io::stdin() ))
  }

  pub fn from_file(path: &str) -> Result<InputReader> {
    let reader = Box::new( File::open(path)? );
    Ok(InputReader::from_reader(reader))
  }

  pub fn from_reader(reader: Box<dyn Read>) -> InputReader {
    InputReader {
      reader,
      buf: vec![0; 1 << 16],
      bytes_read: 0,
      current_index: 0,
      str_buf: String::with_capacity(1 << 8),
    }
  }
}

// public instance methods
impl InputReader {
  pub fn set_buf_size(&mut self, buf_size: usize) -> Result<()> {
    if buf_size < self.bytes_read {
      return io_error!("Data loss while shrinking buffer!");
    }
    self.buf.resize(buf_size, 0);
    Ok(())
  }

  pub fn has_more(&mut self) -> Result<bool> {
    self.ensure_buffer()?;
    Ok(self.bytes_read > 0)
  }

  pub fn next_word(&mut self) -> Result<&str> {
    self.consume_until(|c| c.is_ascii_graphic())?;

    self.str_buf.clear();
    while self.peek().is_ascii_graphic() {
      self.str_buf.push(self.peek());
      self.consume();
      if !self.has_more()? { break; }
    }
    Ok(&self.str_buf)
  }

  pub fn next_line(&mut self) -> Result<&str> {
    if self.peek() == '\n' {
      self.consume();
    }
    self.assert_has_more()?;
    self.str_buf.clear();
    while self.peek() != '\n' {
      self.str_buf.push(self.peek());
      self.consume();
      if !self.has_more()? { break; }
    }
    Ok(&self.str_buf)
  }

  pub fn next_usize(&mut self) -> Result<usize> {
    self.consume_until(|c| c.is_ascii_digit())?;

    let mut num = 0;
    while self.peek().is_ascii_digit() {
      let digit = self.peek() as usize - '0' as usize;
      num = num * 10 + digit;
      self.consume();
      if !self.has_more()? { break; }
    }
    Ok(num)
  }

  pub fn next_i64(&mut self) -> Result<i64> {
    let sign = self.consume_until_signed_num()?;
    Ok(self.next_usize()? as i64 * sign)
  }

  pub fn next_f64(&mut self) -> Result<f64> {
    let sign = self.consume_until_signed_num()? as f64;
    let num: f64 = self.next_word()?.parse().unwrap();
    Ok(num * sign)
  }

  pub fn next_u8(&mut self)  -> Result<u8>  { Ok(self.next_usize()? as u8)  }
  pub fn next_u16(&mut self) -> Result<u16> { Ok(self.next_usize()? as u16) }
  pub fn next_u32(&mut self) -> Result<u32> { Ok(self.next_usize()? as u32) }
  pub fn next_u64(&mut self) -> Result<u64> { Ok(self.next_usize()? as u64) }

  pub fn next_i8(&mut self)  -> Result<i8>  { Ok(self.next_i64()? as i8)  }
  pub fn next_i16(&mut self) -> Result<i16> { Ok(self.next_i64()? as i16) }
  pub fn next_i32(&mut self) -> Result<i32> { Ok(self.next_i64()? as i32) }

  pub fn next_f32(&mut self) -> Result<f32> { Ok(self.next_f64()? as f32) }
}

// private instance methods
impl InputReader {
  fn peek(&self) -> char { self.buf[self.current_index] as char }

  fn consume(&mut self) { self.current_index += 1; }

  fn ensure_buffer(&mut self) -> Result<()> {
    if self.current_index >= self.bytes_read {
      self.bytes_read = self.reader.read(&mut self.buf[..])?;
      self.current_index = 0;
    }
    Ok(())
  }

  fn assert_has_more(&mut self) -> Result<()> {
    if !self.has_more()? {
      return io_error!("Reached end of input");
    }
    Ok(())
  }

  fn consume_until<F: Fn(char) -> bool>(&mut self, test: F) -> Result<()> {
    while !test(self.peek()) {
      self.consume();
      self.assert_has_more()?;
    }
    Ok(())
  }

  fn consume_until_signed_num(&mut self) -> Result<i64> {
    let mut sign = 1;
    loop {
      self.consume_until(|c| c.is_ascii_digit() || c == '-')?;

      if self.peek() != '-' { break; }
      self.consume();
      self.assert_has_more()?;

      // need to check that the next char after
      // '-' is actually a digit
      if self.peek().is_ascii_digit() {
        sign = -1;
        break;
      }
    }
    Ok(sign)
  }
}
