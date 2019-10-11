use std::io::{self, Read, Result, Error};

pub struct InputReader {
  buf: Vec<u8>,
  bytes_read: usize,
  current_index: usize,
}

// Constructors
impl InputReader {
  pub fn new() -> Result<InputReader> { InputReader::with_buf_size(1 << 16) }

  pub fn with_buf_size(buf_size: usize) -> Result<InputReader> {
    let mut input = InputReader {
      buf: vec![0; buf_size],
      bytes_read: 0,
      current_index: 0,
    };
    input.ensure_buffer()?;
    Ok(input)
  }
}

// Public methods
impl InputReader {
  pub fn next_usize(&mut self) -> Result<usize> {
    self.consume_until(|c| c.is_ascii_digit())?;

    let mut num = 0;
    loop {
      let c = self.peek();
      if !c.is_ascii_digit() {
        return Ok(num);
      }
      let digit = c as usize - '0' as usize;
      num = num * 10 + digit;
      self.consume()?;
    }
  }

  pub fn next_i64(&mut self) -> Result<i64> {
    loop {
      self.consume_until(|c| c.is_ascii_digit() || c == '-')?;

      let mut sign = 1;
      if self.peek() == '-' {
        sign = -1;
        self.consume()?;
        if !self.peek().is_ascii_digit() {
          continue;
        }
      }
      return Ok(self.next_usize()? as i64 * sign)
    }
  }

  pub fn next_f64(&mut self) -> Result<f64> { Ok(self.next_word()?.parse().unwrap()) }

  pub fn next_word(&mut self) -> Result<String> {
    self.consume_until(|c| c.is_ascii_graphic())?;

    let mut word = String::new();
    loop {
      let c = self.peek();
      if c.is_whitespace() {
        return Ok(word);
      }
      word.push(c);
      self.consume()?;
    }
  }

  pub fn next_line(&mut self) -> Result<String> {
    let mut line = String::new();
    loop {
      let c = self.peek();
      if c == '\n' {
        return Ok(line);
      }
      line.push(c);
      self.consume()?;
    }
  }

  pub fn next_u32(&mut self) -> Result<u32> { Ok(self.next_usize()? as u32) }
  pub fn next_u64(&mut self) -> Result<u64> { Ok(self.next_usize()? as u64) }
  pub fn next_i32(&mut self) -> Result<i32> { Ok(self.next_i64()?   as i32) }
  pub fn next_f32(&mut self) -> Result<f32> { Ok(self.next_f64()?   as f32) }
}

// Private methods
impl InputReader {
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

  fn peek(&self) -> char { self.buf[self.current_index] as char }

  fn consume(&mut self) -> Result<()> {
    self.current_index += 1;
    self.ensure_buffer()?;
    Ok(())
  }

  fn consume_until<F: Fn(char) -> bool>(&mut self, testFn: F) -> Result<()> {
    while !testFn(self.peek()) {
      self.consume()?;
    }
    Ok(())
  }
}

fn main() -> Result<()> {
  let mut input = InputReader::new()?;
  println!("{}\n{}\n{}", input.next_u32()?, input.next_word()?, input.next_i32()?);
  Ok(())
}
