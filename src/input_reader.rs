
use std::io::{self, Read, Result, Error};

pub struct InputReader {
  buf: Vec<u8>,
  bytes_read: usize,
  current_index: usize,
}

impl InputReader {
  fn new() -> Result<InputReader> { InputReader::with_buf_size(1 << 16) }

  fn with_buf_size(buf_size: usize) -> Result<InputReader> {
    let mut input = InputReader {
      buf: vec![0; buf_size],
      bytes_read: 0,
      current_index: 0,
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

  fn peek_next(&self) -> char { self.buf[self.current_index] as char }

  fn consume(&mut self) -> Result<()> {
    self.current_index += 1;
    self.ensure_buffer()?;
    Ok(())
  }

  fn consume_until<F: Fn(char) -> bool>(&mut self, test: F) -> Result<()> {
    loop {
      if test(self.peek_next()) { return Ok(()); }
      self.consume()?;
    }
  }

  fn next_u32(&mut self) -> Result<u32> {
    self.consume_until(|c| c.is_ascii_digit())?;
    let mut num = 0;
    loop {
      match self.peek_next() {
        '0'..='9' => {
          let digit = self.peek_next() as u32 - '0' as u32;
          num = num * 10 + digit;
        },
        _ => return Ok(num),
      };
      self.consume()?;
    }
  }

  fn next_i32(&mut self) -> Result<i32> {
    self.consume_until(|c| c.is_ascii_digit() || c == '-')?;
    let mut sign = 1;
    let mut num = 0;
    if self.peek_next() == '-' {
      sign = -1;
      self.consume()?;
    }

    loop {
      match self.peek_next() {
        '0'..='9' => {
          let digit = self.peek_next() as i32 - '0' as i32;
          num = num * 10 + digit;
        },
        _ => return Ok(sign * num),
      };
      self.consume()?;
    }
  }

  fn next_word(&mut self) -> Result<String> {
    self.consume_until(|c| c.is_ascii_graphic())?;
    let mut word = String::new();
    loop {
      let c = self.peek_next();
      if c.is_whitespace() {
        return Ok(word);
      }
      word.push(c);
      self.consume()?;
    }
  }
}

fn main() -> Result<()> {
  let mut input = InputReader::new()?;
  println!("{}\n{}\n{}", input.next_u32()?, input.next_word()?, input.next_i32()?);
  Ok(())
}
