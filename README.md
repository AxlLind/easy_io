# InputReader
This is a fast, easy to use, one-file class for reading numbers and words from either stdin, a file, or any other reader.

The main use of this class is in competitive programming. Reading particularly numbers from stdin via the regular `io::stdin` class is not very convenient. In competitive programming you want to be able to easily get the next number in the stream or next word since you know the exact format of the input before hand. This class makes that trivial while also being fast.

## Usage
The simplest use-case for this in competitive programming is to download the file from [here](https://github.com/AxlLind/InputReader/blob/master/src/input_reader.rs). Then simply put it in the same folder as your solution and import it like below. Below is a simple example of using the class:

```Rust
// import the class
mod input_reader;
use input_reader::InputReader;

// ...

fn main() -> std::io::Result<()> {
  // Create a reader from stdin
  let mut input = InputReader::new();

  // ... or from a file.
  let mut input = InputReader::from_file("input.txt")?;

  // ... or from any class that implements the Read trait.
  // The reader needs to be wrapped in a Box.
  // In this example from a tcp stream:
  let tcp_stream = TcpStream::connect("127.0.0.1:34254");
  let mut input = InputReader::from_reader(Box::new(tcp_stream))?;

  // Read numbers and words from the input source simply like this.
  let x: usize = input.next_usize()?;
  let y: i64 = input.next_i64()?;
  let z: f32 = input.next_f32()?;
  let word: String = input.next_word()?;
  let line: String = input.next_line()?;
  Ok(())
}
```

## Public methods
### Constructors
```Rust
// Constructs an InputReader which reads from stdin.
InputReader::new() -> InputReader
```

```Rust
// Constructs an InputReader which reads from the file at the given path.
// Note that this returns a result since it will try to open the file.
InputReader::from_file(path: &str) -> Result<InputReader>
```

```Rust
// Constructs an InputReader that reads from the given reader.
InputReader::from_reader(reader: Box<dyn Read>) -> InputReader
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

InputReader::next_word(&mut self) -> Result<String>
InputReader::next_line(&mut self) -> Result<String>
```

### Other instance methods
```Rust
// Returns Ok(true) if there is more data to be read, Ok(false) otherwise.
InputReader::has_more(&mut self) -> Result<bool>

// Changes the internal buffer size. Default: 2^16 bytes
// Will return error if shrinking the buffer will cause data loss.
InputReader::set_buf_size(&mut self, buf_size: usize) -> Result<()>
```

## :warning: Limitations
This class sacrifices some functionality for performance:
- This does **not** support UTF8 strings. It will treat each byte in the input source as a separate character. This is a significant speed up and in competitive programming almost always only ascii is used anyway.
- It will not do any validation on the size of numbers before trying to fit them in a `u32` for example. This is also fine for competitive programming since number bounds are usually given.
