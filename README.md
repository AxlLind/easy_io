# easy_io
Two structs `InputReader` and `OutputWriter` for fast and convenient IO in Rust.

The main use of these structs is in competitive programming or [Kattis](https://open.kattis.com/) style problems. Reading particularly numbers via `io::stdin()` is not very convenient. In competitive programming you want to be able to easily get the next number or word in the stream since you know the exact format of the input before hand. The `InputReader` struct makes that trivial while also being fast, ensuring IO is not the bottleneck for your solution.

Regular output in Rust via `println!()` can also be problematic since it is line buffered. This can make output can be surprisingly slow. The `OutputWriter` struct buffers all output until it is flushed/dropped which results in a significant performance improvement.

To use these in competitive programming I would simply download [`input_reader.rs`](https://github.com/AxlLind/easy_io/blob/master/src/input_reader.rs) and/or [`output_reader.rs`](https://github.com/AxlLind/easy_io/blob/master/src/output_writer.rs). Put them in the same folder as your solution and import it like below.

This was inspired by [this amazing](https://github.com/williamfiset/FastJavaIO) Java class but written completely separately.

# InputReader
## Usage
```Rust
// import it
mod input_reader;
use input_reader::InputReader;
// ... or if via crates.io
use easy_io::InputReader

fn main() {
  // Create a reader from stdin
  let mut input = InputReader::new();

  // ... or from a file
  let mut input = InputReader::from_file("input.txt");

  // ... or from any struct that implements the Read trait.
  let tcp_stream = TcpStream::connect("127.0.0.1:34254");
  let mut input = InputReader::from_reader(tcp_stream);

  // Read numbers and words from the input source simply like this.
  let x: usize = input.next();
  let y: i64 = input.next();
  let z = input.next::<f64>();
  let word: String = input.next();
  let line: String = input.next_line();
}
```

## :warning: Limitations
This struct sacrifices some functionality/correctness for performance and convenience:
- Results are unwrapped internally so that the API is much simpler. In competitive programming you will not recover from any IO error anyway.
- UTF8 strings are **not** supported. The `InputReader` will treat each byte in the input source as a separate character. This is a significant speed up and in competitive programming only ascii is almost always used anyway.
- It will not do any validation on the size of numbers before trying to fit them in a `u8` for example. This is also fine for competitive programming since number bounds are usually given.
- It only parses decimal notation for numbers, not hexadecimal or scientific notation for example.
- It will not parse special float values like `NaN` or `Infinity`.

## Public methods
### Constructors
```Rust
// InputReader that reads from stdin.
InputReader::new() -> Self

// InputReader that reads from the file at the given path.
InputReader::from_file(path: &str) -> Self

// InputReader that reads from the given reader.
InputReader::from_reader(reader: R) -> Self
```

### The `next()` method
```Rust
fn next<T: InputReadable>(&mut self) -> T;

// In many cases, the compiler can figure out the type for you.
// Other times you need to supply the type like so:
let a: u32 = input.next();
let b = input.next::<f64>();
```

This method is how you read most things from the input source. The following types implement the `InputReadable` trait and are thus usable with this function.

```Rust
u64, u32, u16, u8, usize
i64, u32, i16, i8, isize
f64, f32
char, String
```

### Other instance methods
```Rust
// Returns the next line from the input source.
InputReader::next_line(&mut self) -> String

// Returns true if there is more data to be read from the input source.
InputReader::has_more(&mut self) -> bool

// Changes the internal buffer size. Default: 2^16 bytes
InputReader::set_buf_size(&mut self, buf_size: usize)
```

# OutputWriter
This struct will simply buffer all output until the until the writer is dropped (or the `flush` is called manually).

## Usage
```Rust
// import it
mod output_writer;
use output_writer::OutputWriter;
// ... or if via crates.io
use easy_io::OutputWriter

fn main() -> Result<()> {
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
  Ok(())
}
```

## Public methods
This class implements the `Write` trait. This is so we can utilize the `write!` and `writeln!` macros for easy formatting. This means several more methods are available on the struct though. See documentation [here](https://doc.rust-lang.org/std/io/trait.Write.html).

### Constructors
```Rust
// OutputWriter that writes to stdout.
OutputWriter::new() -> Self

// OutputWriter that writes to the file at the given path.
OutputWriter::from_file(path: &str) -> Self

// OutputWriter that writes to the given writer.
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
