/*
  A fast and dead-simple reader for competitive programming in Rust

  Author: Axel Lindeberg, github.com/AxlLind
  Repository: https://github.com/AxlLind/easy_io
  License: MIT
  2020
*/
#![allow(dead_code)]
use std::io::{self, Read, Stdin};
use std::fs::File;

const EOF: &str = "InputReader: Reached end of input!";

pub struct InputReader<R: Read> {
  reader: R,
  buf: Vec<u8>,
  bytes_read: usize,
  current_index: usize,
}

impl InputReader<Stdin> {
  pub fn new() -> Self {
    Self::from_reader(io::stdin())
  }
}

impl InputReader<File> {
  pub fn from_file(path: &str) -> Self {
    Self::from_reader(File::open(path).unwrap())
  }
}

impl<R: Read> InputReader<R> {
  pub fn from_reader(reader: R) -> Self {
    Self {
      reader,
      buf: vec![0; 1 << 16],
      bytes_read: 0,
      current_index: 0,
    }
  }

  pub fn next<T: InputReadable>(&mut self) -> T {
    T::from_input(self)
  }

  pub fn next_line(&mut self) -> String {
    assert!(self.has_more(), EOF);
    let mut line = String::new();
    while self.peek() != '\n' {
      line.push(self.peek());
      self.consume();
      if !self.has_more() { break; }
    }
    self.consume(); // consume '\n'
    line
  }

  pub fn has_more(&mut self) -> bool {
    if self.current_index >= self.bytes_read {
      self.bytes_read = self.reader.read(&mut self.buf[..]).unwrap();
      self.current_index = 0
    }
    self.bytes_read > 0
  }

  pub fn set_buf_size(&mut self, buf_size: usize) {
    self.buf.resize(buf_size, 0);
  }

  fn peek(&self) -> char { self.buf[self.current_index] as char }

  fn consume(&mut self) { self.current_index += 1; }

  fn pop_digit(&mut self) -> u64 {
    let c = self.peek();
    self.consume();
    c as u64 - '0' as u64
  }

  fn consume_until<F: Fn(char) -> bool>(&mut self, test: F) {
    loop {
      assert!(self.has_more(), EOF);
      if test(self.peek()) { return; }
      self.consume();
    }
  }

  fn consume_until_sign(&mut self) -> i64 {
    loop {
      self.consume_until(|c| c.is_ascii_digit() || c == '-');
      if self.peek() != '-' { return 1; }

      self.consume();
      assert!(self.has_more(), EOF);
      if self.peek().is_ascii_digit() { return -1; }
    }
  }
}

pub trait InputReadable {
  fn from_input<R: Read>(input: &mut InputReader<R>) -> Self;
}

impl InputReadable for u64 {
  fn from_input<R: Read>(input: &mut InputReader<R>) -> Self {
    input.consume_until(|c| c.is_ascii_digit());
    let mut num = 0;
    while input.peek().is_ascii_digit() {
      num = num * 10 + input.pop_digit();
      if !input.has_more() { break; }
    }
    num
  }
}

impl InputReadable for i64 {
  fn from_input<R: Read>(input: &mut InputReader<R>) -> Self {
    let sign = input.consume_until_sign();
    u64::from_input(input) as i64 * sign
  }
}

impl InputReadable for f64 {
  fn from_input<R: Read>(input: &mut InputReader<R>) -> Self {
    let sign = input.consume_until_sign() as f64;
    let mut num = 0.0;
    while input.peek().is_ascii_digit() {
      num = num * 10.0 + input.pop_digit() as f64;
      if !input.has_more() { break; }
    }

    let mut factor = 1.0;
    if input.peek() == '.' {
      input.consume();
      while input.has_more() && input.peek().is_ascii_digit() {
        num = num * 10.0 + input.pop_digit() as f64;
        factor *= 10.0;
      }
    }
    sign * num / factor
  }
}

impl InputReadable for String {
  fn from_input<R: Read>(input: &mut InputReader<R>) -> Self {
    input.consume_until(|c| c.is_ascii_graphic());
    let mut word = String::new();
    while input.peek().is_ascii_graphic() {
      word.push(input.peek());
      input.consume();
      if !input.has_more() { break; }
    }
    word
  }
}

impl InputReadable for char {
  fn from_input<R: Read>(input: &mut InputReader<R>) -> Self {
    input.consume_until(|c| c.is_ascii_graphic());
    let c = input.peek();
    input.consume();
    c
  }
}

macro_rules! impl_readable_from {
  ($A:ty, [$($T:ty),+]) => {
    $(impl InputReadable for $T {
      fn from_input<R: Read>(input: &mut InputReader<R>) -> Self {
        <$A>::from_input(input) as $T
      }
    })+
  };
}
impl_readable_from!{ u64, [u32, u16, u8, usize] }
impl_readable_from!{ i64, [i32, i16, i8, isize] }
impl_readable_from!{ f64, [f32] }
