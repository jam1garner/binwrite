# binwrite

A Rust crate for helping write structs as binary data using ✨macro magic✨


## Usage

BinWrite uses a derive macro for declaratively defining binary writing methods for structs.

### Basic Example

```rust
use binwrite::BinWrite;

#[derive(BinWrite)]
#[binwrite(little)]
struct Rect {
    x: i32,
    y: i32,
    #[binwrite(big)]
    size: (u16, u16),
}

fn main() {
    let rects = vec![
        Rect { x: 1, y: -2, size: (3, 4) },
        Rect { x: 20, y: 4, size: (5, 7) }
    ];
    let mut bytes = vec![];
    rects.write(&mut bytes).unwrap();
    assert_eq!(
        bytes,
        vec![
        //  [  x (little endian) ]  [  y (little endian) ]  [ size.0 ]  [ size.1 ]
            0x01, 0x00, 0x00, 0x00, 0xFE, 0xFF, 0xFF, 0xFF, 0x00, 0x03, 0x00, 0x04,
            0x14, 0x00, 0x00, 0x00, 0x04, 0x00, 0x00, 0x00, 0x00, 0x05, 0x00, 0x07,
        ]
    );
}
```

more examples can be found [in the BinWrite documentation.](https://docs.rs/binwrite/0.1/binwrite/trait.BinWrite.html)
