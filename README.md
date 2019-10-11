# InputReader
This is a fast and easy to use, one-file class for reading numbers and words from either stdin, a file, or any other reader. The main use of this class is in competitive programming.

Reading particularly numbers from stdin via the regular `io::stdin` class is not very convenient. In competitive programming you want to be able to easily get the next number in the stream or next word since you know the exact format of the input before hand. This class makes that trivial while also being fast.

## Usage
The simplest use-case for this in competitive programming is to download the file from [here](https://github.com/AxlLind/InputReader/blob/master/src/input_reader.rs). Then simply put it in the same folder as your solution and import it like below. Below is a simple example of using the class:

```Rust
// import the class
mod input_reader;
use input_reader::InputReader;

// ...

fn main() -> std::io::Result<()> {
  // Create a reader from stdin.
  let mut input = InputReader::new()?;

  // Create a reader from a file.
  let mut input = InputReader::from_file("input.txt")?;

  // Create a reader from any class that implements the Read trait.
  // In this example from a tcp stream. To do this you explicitly need
  // to wrap it in a Box, i.e Box::new(object).
  let tcp_stream = TcpStream::connect("127.0.0.1:34254")?;
  let mut input = InputReader::from_reader(Box::new(tcp_stream))?;

  // Read numbers and words from the input simply like this.
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
InputReader::new() -> Result<InputReader>
```

```Rust
// Constructs an InputReader which reads from the file at the given path.
InputReader::from_file(path: &str) -> Result<InputReader>
```

```Rust
// Constructs an InputReader that reads from the given reader.
InputReader::from_reader(reader: Box<dyn Read>) -> Result<InputReader>
```

### Reader methods
The following methods are pretty self-explanatory. They read the next *thing* from the input source.

```Rust
InputReader::next_usize(&mut self) -> Result<usize>
InputReader::next_i32(&mut self) -> Result<i32>
InputReader::next_i64(&mut self) -> Result<i64>
InputReader::next_u32(&mut self) -> Result<u32>
InputReader::next_u64(&mut self) -> Result<u64>
InputReader::next_f32(&mut self) -> Result<f32>
InputReader::next_f64(&mut self) -> Result<f64>
InputReader::next_word(&mut self) -> Result<String>
InputReader::next_line(&mut self) -> Result<String>
```

### Instance methods
```Rust
// Changes the internal buffer size of the InputReader. Default: 2^16 bytes
InputReader::set_buf_size(&mut self, buf_size: usize)
```
