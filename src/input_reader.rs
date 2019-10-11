use std::io::{self, Read, Result, Error};

trait InputReader {
  // implementations
  fn peek(&self) -> char;
  fn consume(&mut self) -> Result<()>;
  fn str_buf(&mut self) -> &mut String;

  // defaults
  fn consume_until<F: Fn(char) -> bool>(&mut self, test: F) -> Result<()> {
    while !test(self.peek()) { self.consume()?; }
    Ok(())
  }

  fn next_word(&mut self) -> Result<String> {
    self.consume_until(|c| c.is_ascii_graphic())?;

    self.str_buf().clear();
    while self.peek().is_ascii_graphic() {
      let c = self.peek();
      self.str_buf().push(c);
      self.consume()?;
    }
    Ok(self.str_buf().clone())
  }

  fn next_line(&mut self) -> Result<String> {
    self.str_buf().clear();
    while self.peek() != '\n' {
      let c = self.peek();
      self.str_buf().push(c);
      self.consume()?;
    }
    Ok(self.str_buf().clone())
  }

  fn next_usize(&mut self) -> Result<usize> {
    self.consume_until(|c| c.is_ascii_digit())?;

    let mut num = 0;
    while self.peek().is_ascii_digit() {
      let digit = self.peek() as usize - '0' as usize;
      num = num * 10 + digit;
      self.consume()?;
    }
    Ok(num)
  }

  fn next_i64(&mut self) -> Result<i64> {
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

  fn next_f64(&mut self) -> Result<f64> {
    self.consume_until(|c| c.is_ascii_digit() || c == '-')?;
    Ok(self.next_word()?.parse().unwrap())
  }

  fn next_u32(&mut self) -> Result<u32> { Ok(self.next_usize()? as u32) }
  fn next_u64(&mut self) -> Result<u64> { Ok(self.next_usize()? as u64) }
  fn next_i32(&mut self) -> Result<i32> { Ok(self.next_i64()?   as i32) }
  fn next_f32(&mut self) -> Result<f32> { Ok(self.next_f64()?   as f32) }
}

pub struct StdinReader {
  buf: Vec<u8>,
  bytes_read: usize,
  current_index: usize,
  str_buf: String,
}

impl StdinReader {
  pub fn new() -> Result<StdinReader> { StdinReader::with_buf_size(1 << 16) }

  pub fn with_buf_size(buf_size: usize) -> Result<StdinReader> {
    let mut input = StdinReader {
      buf: vec![0; buf_size],
      bytes_read: 0,
      current_index: 0,
      str_buf: String::with_capacity(1 << 8),
    };
    input.ensure_buffer()?;
    Ok(input)
  }

  fn ensure_buffer(&mut self) -> Result<()> {
    if self.current_index == self.bytes_read {
      self.current_index = 0;
      self.bytes_read = io::stdin().read(&mut self.buf[..])?;
      if self.bytes_read == 0 {
        return Err(Error::new(io::ErrorKind::Other, "InputReader: Could not read more bytes"));
      }
    }
    Ok(())
  }
}

impl InputReader for StdinReader {
  fn peek(&self) -> char { self.buf[self.current_index] as char }

  fn consume(&mut self) -> Result<()> {
    self.current_index += 1;
    self.ensure_buffer()
  }

  fn str_buf(&mut self) -> &mut String { &mut self.str_buf }
}

fn main() -> Result<()> {
  let mut input = StdinReader::new()?;
  println!("{}\n{}\n{}", input.next_u32()?, input.next_word()?, input.next_i32()?);
  Ok(())
}
