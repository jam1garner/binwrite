# binwrite

A Rust crate for helping write structs as binary data using ✨macro magic✨


## Usage

The idea behind binwrite is using a derive macro for declaratively defining binary writing.

### Basic Example

```rust
use binwrite::BinWrite;

#[derive(BinWrite)]
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let point = Point { x: 1, y: -2 };
    let mut bytes = vec![];
    point.write(&mut bytes).unwrap();

    assert_eq!(bytes, vec![1, 0, 0, 0, 0xFE, 0xFF, 0xFF, 0xFF]);
}
```

more examples can be found [in the BinWrite documentation.](https://docs.rs/binwrite/0.1/binwrite/trait.BinWrite.html)
