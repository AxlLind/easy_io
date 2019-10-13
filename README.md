# EasyIO.rs
Two structs `InputReader` and `OutputWriter` for fast and convenient IO in Rust.

The main use of these structs is in competitive programming or [Kattis](https://open.kattis.com/) style problems. Reading particularly numbers via `io::stdin()` is not very convenient. In competitive programming you want to be able to easily get the next number or word in the stream since you know the exact format of the input before hand. The `InputReader` struct makes that trivial while also being fast.

Regular output in Rust via `println!()` can also be problematic since it is line buffered. This can make output can be surprisingly slow. The `OutputWriter` struct buffers all output which results in a much better performance.

To use these in competitive programming simply download [`input_reader.rs`](https://github.com/AxlLind/EasyIO.rs/blob/master/src/input_reader.rs) or [`output_reader.rs`](https://github.com/AxlLind/EasyIO.rs/blob/master/src/output_writer.rs). Put them in the same folder as your solution and import it like below.

This was inspired by [this amazing](https://github.com/williamfiset/FastJavaIO) Java class but written completely separately.

# InputReader
## Usage
```Rust
// import it
mod input_reader;
use input_reader::InputReader;

fn main() -> std::io::Result<()> {
  // Create a reader from stdin
  let mut input = InputReader::new();

  // ... or from a file
  let mut input = InputReader::from_file("input.txt")?;

  // ... or from any struct that implements the Read trait.
  // The reader needs to be wrapped in a Box.
  // In this example from a tcp stream:
  let tcp_stream = Box::new( TcpStream::connect("127.0.0.1:34254") );
  let mut input = InputReader::from_reader(tcp_stream);

  // Read numbers and words from the input source simply like this.
  let x: usize = input.next_usize()?;
  let y: i64 = input.next_i64()?;
  let z: f32 = input.next_f32()?;
  let word: String = input.next_word()?.to_string();
  let line: String = input.next_line()?.to_string();
  Ok(())
}
```

## :warning: Limitations
This struct sacrifices some functionality for performance:
- This does **not** support UTF8 strings. It will treat each byte in the input source as a separate character. This is a significant speed up and in competitive programming almost always only ascii is used anyway.
- It will not do any validation on the size of numbers before trying to fit them in a `u32` for example. This is also fine for competitive programming since number bounds are usually given.

## Public methods
### Constructors
```Rust
// Constructs an InputReader which reads from stdin.
InputReader::new() -> Self
```

```Rust
// Constructs an InputReader which reads from the file at the given path.
// Note that this returns a result since it will try to open the file.
InputReader::from_file(path: &str) -> Result<Self>
```

```Rust
// Constructs an InputReader that reads from the given reader.
InputReader::from_reader(reader: Box<dyn Read>) -> Self
```

### Reader methods
The following methods are pretty self-explanatory. They read the next *thing* from the input source.

```Rust
InputReader::next_usize(&mut self) -> Result<usize>

InputReader::next_i8(&mut self)  -> Result<i8>
InputReader::next_i16(&mut self) -> Result<i16>
InputReader::next_i32(&mut self) -> Result<i32>
InputReader::next_i64(&mut self) -> Result<i64>

InputReader::next_u8(&mut self)  -> Result<u8>
InputReader::next_u16(&mut self) -> Result<u16>
InputReader::next_u32(&mut self) -> Result<u32>
InputReader::next_u64(&mut self) -> Result<u64>

InputReader::next_f32(&mut self) -> Result<f32>
InputReader::next_f64(&mut self) -> Result<f64>

// Note that it will not include the newline char
InputReader::next_line(&mut self) -> Result<&str>
InputReader::next_word(&mut self) -> Result<&str>
```

The two string methods return a `&str` instead of a `String` for optimization reasons. If you need a `String` that you own you can copy it by doing `input.next_word()?.to_string()`.

### Other instance methods
```Rust
// Returns Ok(true) if there is more data to be read, Ok(false) otherwise.
InputReader::has_more(&mut self) -> Result<bool>

// Changes the internal buffer size. Default: 2^16 bytes
// Will return error if shrinking the buffer will cause data loss.
InputReader::set_buf_size(&mut self, buf_size: usize) -> Result<()>
```

# OutputWriter
## Usage
```Rust
// import it
mod output_writer;
use output_writer::OutputWriter;

fn main() -> std::io::Result<()> {
  // Create a writer from stdout
  let mut output = OutputWriter::new();

  // ... or from a file
  let mut input = OutputWriter::from_file("input.txt")?;

  // ... or from any struct that implements the Write trait.
  // The writer needs to be wrapped in a Box.
  // In this example from a tcp stream:
  let tcp_stream = Box::new( TcpStream::connect("127.0.0.1:34254") );
  let mut input = OutputWriter::from_writer(tcp_stream);

  // Write to the output source simply like this.
  output.writeln("Hello world!");
  output.write(&format!("{} is a cool number.\n", 1337));

  // Optionally you can manually flush the writer.
  // This will be done automatically when the writer is dropped.
  output.flush()?;
  Ok(())
}
```

## Public methods
### Constructors
```Rust
// Constructs an OutputWriter which writes to stdout.
OutputWriter::new() -> Self
```

```Rust
// Constructs an OutputWriter which writes to the file at the given path.
// Note that this returns a result since it will try to open the file.
OutputWriter::from_file(path: &str) -> Result<Self>
```

```Rust
// Constructs an OutputWriter that writes to the given writer.
OutputWriter::from_writer(reader: Box<dyn Write>) -> Self
```

### Instance methods
```Rust
// Writes the string to the output source.
OutputWriter::write(&mut self, s: &str)

// Convenience method for writing the given string with a newline appended.
OutputWriter::writeln(&mut self, s: &str)

// Flushes the internal buffer and writes it to the output source.
// Note that this is called on drop so you do not have to do it manually.
OutputWriter::flush(&mut self) -> Result<()>
```
