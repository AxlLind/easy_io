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

fn main() {
  // Create a reader from stdin
  let mut input = InputReader::new();

  // ... or from a file
  let mut input = InputReader::from_file("input.txt");

  // ... or from any struct that implements the Read trait.
  let tcp_stream = TcpStream::connect("127.0.0.1:34254");
  let mut input = InputReader::from_reader(tcp_stream);

  // Read numbers and words from the input source simply like this.
  let x = input.next_usize();
  let y = input.next_i64();
  let z = input.next_f32();
  let word: String = input.next_word().to_string();
  let line: String = input.next_line().to_string();
}
```

## :warning: Limitations
This struct sacrifices some functionality/correctness for performance and convenience:
- Results are unwrapped internally so that the API is much simpler. In competitive programming you will not recover from any IO error anyway.
- UTF8 strings are **not** supported. The `InputReader` will treat each byte in the input source as a separate character. This is a significant speed up and in competitive programming only ascii is almost always used anyway.
- It will not do any validation on the size of numbers before trying to fit them in a `u8` for example. This is also fine for competitive programming since number bounds are usually given.
- Only parses decimal notation for numbers, not hexadecimal for example.
- It will not parse special float values like `NaN` or `Infinity`.

## Public methods
### Constructors
```Rust
// Constructs an InputReader which reads from stdin.
InputReader::new() -> Self
```

```Rust
// Constructs an InputReader which reads from the file at the given path.
InputReader::from_file(path: &str) -> Self
```

```Rust
// Constructs an InputReader that reads from the given reader.
InputReader::from_reader(reader: R) -> Self
```

### Reader methods
The following methods are pretty self-explanatory. They read the next *thing* from the input source.

```Rust
InputReader::next_usize(&mut self) -> usize

InputReader::next_u8(&mut self)  -> u8
InputReader::next_u16(&mut self) -> u16
InputReader::next_u32(&mut self) -> u32
InputReader::next_u64(&mut self) -> u64

InputReader::next_i8(&mut self)  -> i8
InputReader::next_i16(&mut self) -> i16
InputReader::next_i32(&mut self) -> i32
InputReader::next_i64(&mut self) -> i64

InputReader::next_f32(&mut self) -> f32
InputReader::next_f64(&mut self) -> f64

// Note that it will not include the newline char
InputReader::next_line(&mut self) -> &str
InputReader::next_word(&mut self) -> &str
InputReader::next_char(&mut self) -> char
```

The two string methods return a `&str` instead of a `String` for optimization reasons. If you need a `String` that you own you can copy it by doing `input.next_word().to_string()`.

### Other instance methods
```Rust
// Returns true if there is more data to be read from the input source.
InputReader::has_more(&mut self) -> bool

// Changes the internal buffer size. Default: 2^16 bytes
// Will panic if shrinking the buffer would cause data loss.
InputReader::set_buf_size(&mut self, buf_size: usize)
```

# OutputWriter
This struct will simply buffer all output until the function `flush` is called which also happens automatically when the writer is dropped.

## Usage
```Rust
// import it
mod output_writer;
use output_writer::OutputWriter;

fn main() -> std::io::Result<()> {
  // Create a writer from stdout
  let mut output = OutputWriter::new();

  // ... or from a file
  let mut output = OutputWriter::from_file("output.txt");

  // ... or from any struct that implements the Write trait.
  let tcp_stream = TcpStream::connect("127.0.0.1:34254");
  let mut output = OutputWriter::from_writer(tcp_stream);

  // Write to the output source simply like this.
  // These methods accept any object that implements Display.
  output.println("Hello world!");
  output.prints(1337);
  output.println("is a cool number.");
  // or like this
  output.print(format!("{} is a cool number.\n", 1337));

  // It also implements the Write trait, so you can do this:
  write!(output, "{} formatted!\n", "This is")?;
  writeln!(output, "{} is the answer.", 42)?;

  // You can manually flush the writer. Note that this will
  // be done automatically when the writer is dropped.
  output.flush()?;
  Ok()
}
```

## Public methods
This class implements the `Write` trait. This is so we can utilize the `write!` and `writeln!` macros for easy formatting. This means several more methods are available on the struct though. See documentation [here](https://doc.rust-lang.org/std/io/trait.Write.html).

### Constructors
```Rust
// Constructs an OutputWriter which writes to stdout.
OutputWriter::new() -> Self
```

```Rust
// Constructs an OutputWriter which writes to the file at the given path.
OutputWriter::from_file(path: &str) -> Self
```

```Rust
// Constructs an OutputWriter that writes to the given writer.
OutputWriter::from_writer(writer: W) -> Self
```

### Instance methods
```Rust
// Writes something to the output source.
OutputWriter::print<T: Display>(&mut self, t: T)

// Convenience method for writing something with a space appended.
OutputWriter::prints<T: Display>(&mut self, t: T)

// Convenience method for writing something with a newline appended.
OutputWriter::println<T: Display>(&mut self, t: T)
```
