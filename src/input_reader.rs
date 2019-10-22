/*
  A fast and dead-simple reader for competitive programming in Rust

  Author: Axel Lindeberg, github.com/AxlLind
  Repository: https://github.com/AxlLind/EasyIO.rs
  License: MIT
  2019
*/
#![allow(dead_code)]
use std::io::{self, Read};
use std::fs::File;

pub struct InputReader {
  reader: Box<dyn Read>,
  buf: Vec<u8>,
  bytes_read: usize,
  current_index: usize,
  str_buf: String,
}

// Constructors
impl InputReader {
  pub fn new() -> Self {
    Self::from_reader(Box::new( io::stdin() ))
  }

  pub fn from_file(path: &str) -> Self {
    let file = Box::new( File::open(path).unwrap() );
    Self::from_reader(file)
  }

  pub fn from_reader(reader: Box<dyn Read>) -> Self {
    Self {
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
  pub fn next_word(&mut self) -> &str {
    self.consume_until(|c| c.is_ascii_graphic());

    self.str_buf.clear();
    while self.peek().is_ascii_graphic() {
      self.str_buf.push(self.peek());
      self.consume();
      if !self.has_more() { break; }
    }
    &self.str_buf
  }

  pub fn next_line(&mut self) -> &str {
    self.assert_has_more();

    self.str_buf.clear();
    while self.peek() != '\n' {
      self.str_buf.push(self.peek());
      self.consume();
      if !self.has_more() { break; }
    }
    self.consume(); // consume the newline
    &self.str_buf
  }

  pub fn next_usize(&mut self) -> usize {
    self.consume_until(|c| c.is_ascii_digit());

    let mut num = 0;
    while self.peek().is_ascii_digit() {
      let digit = self.peek() as usize - '0' as usize;
      num = num * 10 + digit;
      self.consume();
      if !self.has_more() { break; }
    }
    num
  }

  pub fn next_i64(&mut self) -> i64 {
    let sign = self.consume_until_signed_num();
    self.next_usize() as i64 * sign
  }

  pub fn next_f64(&mut self) -> f64 {
    let sign = self.consume_until_signed_num() as f64;
    let num: f64 = self.next_word().parse().unwrap();
    num * sign
  }

  pub fn next_u8(&mut self)  -> u8  { self.next_usize() as u8  }
  pub fn next_u16(&mut self) -> u16 { self.next_usize() as u16 }
  pub fn next_u32(&mut self) -> u32 { self.next_usize() as u32 }
  pub fn next_u64(&mut self) -> u64 { self.next_usize() as u64 }
  pub fn next_i8(&mut self)  -> i8  { self.next_i64()   as i8  }
  pub fn next_i16(&mut self) -> i16 { self.next_i64()   as i16 }
  pub fn next_i32(&mut self) -> i32 { self.next_i64()   as i32 }
  pub fn next_f32(&mut self) -> f32 { self.next_f64()   as f32 }

  pub fn has_more(&mut self) -> bool {
    if self.current_index >= self.bytes_read {
      self.bytes_read = self.reader.read(&mut self.buf[..]).unwrap();
      self.current_index = 0
    }
    self.bytes_read > 0
  }

  pub fn set_buf_size(&mut self, buf_size: usize) {
    assert!(buf_size >= self.bytes_read, "InputReader: Data loss while shrinking buffer!");
    self.buf.resize(buf_size, 0);
  }
}

// private instance methods
impl InputReader {
  fn peek(&self) -> char { self.buf[self.current_index] as char }

  fn consume(&mut self) { self.current_index += 1; }

  fn assert_has_more(&mut self) {
    assert!(self.has_more(), "InputReader: Reached end of input!");
  }

  fn consume_until<F: Fn(char) -> bool>(&mut self, test: F) {
    while !test(self.peek()) {
      self.consume();
      self.assert_has_more();
    }
  }

  fn consume_until_signed_num(&mut self) -> i64 {
    let mut sign = 1;
    loop {
      self.consume_until(|c| c.is_ascii_digit() || c == '-');

      if self.peek() != '-' { break; }
      self.consume();
      self.assert_has_more();

      // need to check that the char after
      // '-' is actually a digit
      if self.peek().is_ascii_digit() {
        sign = -1;
        break;
      }
    }
    sign
  }
}
