#![feature(fixed_size_array)]
#![feature(trait_alias)]
use byteorder::{WriteBytesExt, BE, LE, NativeEndian};
use std::io::{Result, Write};

pub mod write_track;
pub mod writers;
mod binwrite_impls;

pub use binwrite_impls::*;

pub trait BinWrite {
    fn write<W: Write>(&self, writer: &mut W) -> std::io::Result<()> {
        self.write_options(writer, &WriterOption::default())
    }

    fn write_options<W: Write>(&self, writer: &mut W, options: &WriterOption) -> Result<()>;
}

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

#[derive(Default, Clone)]
pub struct WriterOption {
    pub endian: Endian,
    // A private field to prevent users from creating/destructuring in a non-forwards compatible
    // manner
    _prevent_creation: ()
}

#[macro_export] macro_rules! writer_option_new {
    ($($field:ident : $val:expr),*$(,)?) => {
        {
            let mut _writer_option = WriterOption::default();
            $(
                _writer_option.$field = $val;
            )*
            _writer_option
        }
    }
}

pub enum OtherOptions {

}

impl Default for Endian {
    fn default() -> Endian {
        Endian::Native
    }
}
