use std::io::{self, Read, Result, Error};
use std::fs::File;

pub struct InputReader {
  reader: Box<dyn Read>,
  buf: Vec<u8>,
  bytes_read: usize,
  current_index: usize,
  str_buf: String,
}

impl InputReader {
  pub fn new() -> Result<InputReader> {
    let reader = Box::new(io::stdin());
    InputReader::from_reader(reader)
  }

  pub fn from_file(path: &str) -> Result<InputReader> {
    let reader = Box::new(File::open(path)?);
    InputReader::from_reader(reader)
  }

  pub fn from_reader(reader: Box<dyn Read>) -> Result<InputReader> {
    let mut input = InputReader {
      reader,
      buf: vec![0; 1 << 16],
      bytes_read: 0,
      current_index: 0,
      str_buf: String::with_capacity(1 << 8),
    };
    input.ensure_buffer()?;
    Ok(input)
  }

  pub fn set_buf_size(&mut self, buf_size: usize) { self.buf.resize(buf_size, 0); }

  pub fn next_word(&mut self) -> Result<String> {
    self.consume_until(|c| c.is_ascii_graphic())?;

    self.str_buf.clear();
    while self.peek().is_ascii_graphic() {
      self.str_buf.push(self.peek());
      self.consume()?;
    }
    Ok(self.str_buf.clone())
  }

  pub fn next_line(&mut self) -> Result<String> {
    self.str_buf.clear();
    while self.peek() != '\n' {
      self.str_buf.push(self.peek());
      self.consume()?;
    }
    Ok(self.str_buf.clone())
  }

  pub fn next_usize(&mut self) -> Result<usize> {
    self.consume_until(|c| c.is_ascii_digit())?;

    let mut num = 0;
    while self.peek().is_ascii_digit() {
      let digit = self.peek() as usize - '0' as usize;
      num = num * 10 + digit;
      self.consume()?;
    }
    Ok(num)
  }

  pub fn next_i64(&mut self) -> Result<i64> {
    let mut sign = 1;
    loop {
      self.consume_until(|c| c.is_ascii_digit() || c == '-')?;

      if self.peek() != '-' { break; }
      self.consume()?;

      if self.peek().is_ascii_digit() {
        sign = -1;
        break;
      }
    }
    Ok(self.next_usize()? as i64 * sign)
  }

  pub fn next_f64(&mut self) -> Result<f64> {
    self.consume_until(|c| c.is_ascii_digit() || c == '-')?;
    Ok(self.next_word()?.parse().unwrap())
  }

  pub fn next_u32(&mut self) -> Result<u32> { Ok(self.next_usize()? as u32) }
  pub fn next_u64(&mut self) -> Result<u64> { Ok(self.next_usize()? as u64) }
  pub fn next_i32(&mut self) -> Result<i32> { Ok(self.next_i64()?   as i32) }
  pub fn next_f32(&mut self) -> Result<f32> { Ok(self.next_f64()?   as f32) }
}

impl InputReader {
  fn ensure_buffer(&mut self) -> Result<()> {
    if self.current_index == self.bytes_read {
      self.current_index = 0;
      self.bytes_read = self.reader.read(&mut self.buf[..])?;
      if self.bytes_read == 0 {
        return Err(Error::new(io::ErrorKind::Other, "InputReader: Could not read more bytes"));
      }
    }
    Ok(())
  }

  fn peek(&self) -> char { self.buf[self.current_index] as char }

  fn consume(&mut self) -> Result<()> {
    self.current_index += 1;
    self.ensure_buffer()
  }

  fn consume_until<F: Fn(char) -> bool>(&mut self, test: F) -> Result<()> {
    while !test(self.peek()) { self.consume()?; }
    Ok(())
  }
}

fn main() -> Result<()> {
  let mut stdin = InputReader::new()?;
  println!("{}\n{}\n{}", stdin.next_u32()?, stdin.next_word()?, stdin.next_i32()?);
  Ok(())
}
