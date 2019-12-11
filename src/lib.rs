//! A Rust crate for helping write structs as binary data using ✨macro magic✨
//! 
//! # Example:
//!```rust
//! use binwrite::BinWrite;
//!
//! #[derive(BinWrite)]
//! #[binwrite(little)]
//! struct Rect {
//!     x: i32,
//!     y: i32,
//!     #[binwrite(big)]
//!     size: (u16, u16),
//! }
//!
//! fn main() {
//!     let rects = vec![
//!         Rect { x: 1, y: -2, size: (3, 4) },
//!         Rect { x: 20, y: 4, size: (5, 7) }
//!     ];
//!     let mut bytes = vec![];
//!     rects.write(&mut bytes).unwrap();
//!     assert_eq!(
//!         bytes,
//!         vec![
//!         //  [  x (little endian) ]  [  y (little endian) ]  [ size.0 ]  [ size.1 ]
//!             0x01, 0x00, 0x00, 0x00, 0xFE, 0xFF, 0xFF, 0xFF, 0x00, 0x03, 0x00, 0x04,
//!             0x14, 0x00, 0x00, 0x00, 0x04, 0x00, 0x00, 0x00, 0x00, 0x05, 0x00, 0x07,
//!         ]
//!     );
//! }
//!```

use byteorder::{WriteBytesExt, BE, LE, NativeEndian};
use std::io::{Result, Write};

/// Derive macro for BinWrite. [Usage here](BinWrite).
pub use binwrite_derive::BinWrite;

/// Module for [WriteTrack\<T\>](write_track::WriteTrack)
pub mod write_track;
/// Built-in special writers (example: C strings)
pub mod writers;
mod binwrite_impls;

pub use binwrite_impls::*;

/// A trait providing the ability to write the struct to a writer
///
/// ### Derive-based example:
/// ```rust
/// use binwrite::BinWrite;
///
/// #[derive(BinWrite)]
/// struct Point {
///     x: i32,
///     y: i32,
/// }
///
/// fn main() {
///     let point = Point { x: 1, y: -2 };
///     let mut bytes = vec![];
///
///     point.write(&mut bytes).unwrap();
///
///     assert_eq!(bytes, vec![1, 0, 0, 0, 0xFE, 0xFF, 0xFF, 0xFF]);
/// }
/// ```
///
/// ### Setting Endianness
/// ```rust
/// use binwrite::BinWrite;
///
/// #[derive(BinWrite)]
/// #[binwrite(big)]
/// struct Foo {
///     bar: u32,
///     bar2: i32,
///
///     #[binwrite(little)]
///     bar3: u32,
/// }
///
/// fn main() {
///     let point = Foo {
///         bar: 1,
///         bar2: -2,
///         bar3: 3
///     };
///     let mut bytes = vec![];
///
///     point.write(&mut bytes).unwrap();
///
///     assert_eq!(bytes, vec![0, 0, 0, 1, 0xFF, 0xFF, 0xFF, 0xFE, 3, 0, 0, 0]);
/// }
/// ```
///
/// ### Using a preprocessor
/// ```rust
/// use binwrite::BinWrite;
///
/// #[derive(BinWrite)]
/// struct Foo {
///     #[binwrite(preprocessor(u32_to_hex_string))]
///     bar: u32,
///     bar2: String,
/// }
///
/// fn u32_to_hex_string(var: u32) -> String {
///     format!("{:X}", var)
/// }
///
/// fn main() {
///     let point = Foo {
///         bar: 0xF00D,
///         bar2: String::from(" looks like food")
///     };
///     let mut bytes = vec![];
///
///     point.write(&mut bytes).unwrap();
///
///     assert_eq!(bytes, b"F00D looks like food");
/// }
/// ```
///
/// ### Using a custom writer
///
/// For more complicated or more reusable serialization methods, you may want to use a custom
/// writer instead of just preprocessing.
/// ```rust
/// use std::io::{Write, Result};
/// use binwrite::{BinWrite, WriterOption};
///
/// #[derive(BinWrite)]
/// struct Foo {
///     vec_without_len: Vec<u8>,
///     #[binwrite(with(write_vec_with_len), big)]
///     vec_with_len: Vec<u8>,
/// }
///
/// pub fn write_vec_with_len<W, T>(vec: &Vec<T>, writer: &mut W, options: &WriterOption) -> Result<()>
///     where W: Write,
///              T: BinWrite,
/// {
///     BinWrite::write_options(&(vec.len() as u32), writer, options)?;
///     BinWrite::write_options(vec, writer, options)
/// }
///
/// fn main() {
///     let point = Foo {
///         vec_without_len: vec![0, 1, 2, 3],
///         vec_with_len: vec![0, 1, 2, 3],
///     };
///     let mut bytes = vec![];
///
///     point.write(&mut bytes).unwrap();
///
///     assert_eq!(bytes, vec![0, 1, 2, 3, 0, 0, 0, 4, 0, 1, 2, 3]);
/// }
/// ```
///
/// ### Built in Writers:
/// Currently supported built in writers:
/// * cstr - "C string" (null terminated string)
/// * utf16 - UTF-16/2 byte wide/Windows string, endianness is used to determine byte order
/// * utf16_null - same as utf16 but with a null terminator
/// * ignore - skip writing this field
/// ```rust
/// use binwrite::BinWrite;
///
/// #[derive(BinWrite)]
/// struct Foo {
///     #[binwrite(cstr)]
///     bar: u32,
///     #[binwrite(cstr)]
///     bar2: String,
///     #[binwrite(ignore)]
///     bar3: u8,
/// }
///
/// fn main() {
///     let point = Foo {
///         bar: 1234,
///         bar2: String::from("this is null terminated"),
///         bar3: 0xFF
///     };
///     let mut bytes = vec![];
///
///     point.write(&mut bytes).unwrap();
///
///     assert_eq!(bytes, b"1234\0this is null terminated\0");
/// }
/// ```
///
/// ### Padding/Alignment
/// binwrite also has the ability to align to the nearest X bytes
/// ```rust
/// use binwrite::BinWrite;
///
/// #[derive(BinWrite)]
/// struct Foo {
///     // For tuples/arrays/vecs/slices of types implementing BinWrite work out of the box
///     // and items will just be written in order.
///     bar: [char; 3],
///     // pad specifies the padding before
///     // pad_after specifiers the padding after
///     #[binwrite(align(8), align_after(0x10))]
///     bar2: String,
/// }
///
/// fn main() {
///     let point = Foo {
///         bar: ['a', 'b', 'c'],
///         bar2: String::from("test string")
///     };
///     let mut bytes = vec![];
///
///     point.write(&mut bytes).unwrap();
///
///     assert_eq!(bytes, b"abc\0\0\0\0\0test string\0\0\0\0\0\0\0\0\0\0\0\0\0");
/// }
/// ```
/// use `pad` and `pad_after` for fixed amounts of padding.
///
/// ### Advanced Preprocessors
/// Using generics/closures, you can make "configurable" and more reusable preprocessors.
///
/// Example (a configurable "add X before writing"):
/// ```rust
/// use binwrite::BinWrite;
///
/// fn add<T: std::ops::Add<Output = T> + Copy>(lhs: T) -> impl Fn(T) -> T {
///     move |rhs| lhs + rhs
/// }
///
/// #[derive(BinWrite)]
/// struct Foo {
///     #[binwrite(preprocessor(add(10)))]
///     bar_u32: u32,
///     #[binwrite(preprocessor(add(-1)))]
///     bar_i64: i64,
/// }
///
/// fn main() {
///     let mut bytes = vec![];
///
///     Foo {
///         bar_u32: 2,
///         bar_i64: 0,
///     }.write(&mut bytes).unwrap();
///
///     assert_eq!(bytes, vec![0xCu8, 0, 0, 0, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF]);
/// }
/// ```
///
/// ### Postprocessors
///
/// Postprocessors are functions which take a `Vec<u8>` (what would normally be written) and
/// produce any type that implements BinWrite, which is then written in place of the bytes.
/// ```rust
/// use binwrite::BinWrite;
///
/// fn not_crc32(bytes: &Vec<u8>) -> u32 {
///     4
/// }
///
/// fn prepend_crc32(bytes: Vec<u8>) -> (u32, Vec<u8>) {
///     (
///         not_crc32(&bytes),
///         bytes
///     )
/// }
///
/// #[derive(BinWrite)]
/// #[binwrite(big)]
/// struct Foo {
///     #[binwrite(postprocessor(prepend_crc32))]
///     bar: u32,
/// }
///
/// fn main() {
///     let mut bytes = vec![];
///
///     Foo {
///         bar: 2,
///     }.write(&mut bytes).unwrap();
///
///     assert_eq!(bytes, vec![0x0u8, 0, 0, 4, 0, 0, 0, 0x2]);
/// }
/// ```
pub trait BinWrite {
    fn write<W: Write>(&self, writer: &mut W) -> std::io::Result<()> {
        self.write_options(writer, &WriterOption::default())
    }

    fn write_options<W: Write>(&self, writer: &mut W, options: &WriterOption) -> Result<()>;
}

/// An enum to represent what endianness to write with
#[derive(Clone, Copy, Debug)]
pub enum Endian {
    Big,
    Little,
    Native,
}

impl Into<String> for &Endian {
    fn into(self) -> String {
        String::from(
            match self {
                Endian::Big => "Big",
                Endian::Little => "Little",
                Endian::Native => "Native",
            }
        )
    }
}

/// Options on how to write. Use [writer_option_new!](writer_option_new) to create a new
/// instance. Manual initialization is not possible to prevent forward compatibility issues.
#[derive(Default, Clone)]
pub struct WriterOption {
    pub endian: Endian,
    /// A private field to prevent users from creating/destructuring in a non-forwards compatible
    /// manner
    _prevent_creation: ()
}

/// Macro for creating a new writer option, with the idea being a non-verbose means of providing a
/// forwards-compatible set of options which uses default values for all provided options.
#[macro_export] macro_rules! writer_option_new {
    ($($field:ident : $val:expr),*$(,)?) => {
        {
            let mut _writer_option = ::binwrite::WriterOption::default();
            $(
                _writer_option.$field = $val;
            )*
            _writer_option
        }
    }
}

impl Default for Endian {
    fn default() -> Endian {
        Endian::Native
    }
}
