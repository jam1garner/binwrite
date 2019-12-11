use super::*;

/// Internal macro for quickly implementing binwrite for types supported by byteorder
macro_rules! binwrite_impl {
    ($(($type_name:ty, $write_func:ident)),*$(,)?) => {
        $(
            impl BinWrite for $type_name {
                fn write_options<W: Write>(&self, writer: &mut W, options: &WriterOption) -> Result<()> {
                    match options.endian {
                        Endian::Big => {
                            writer.$write_func::<BE>(*self)
                        }
                        Endian::Little => {
                            writer.$write_func::<LE>(*self)
                        }
                        Endian::Native => {
                            writer.$write_func::<NativeEndian>(*self)
                        }
                    }
                }
            }
        )*
    }
}

impl BinWrite for char {
    fn write_options<W: Write>(&self, writer: &mut W, _options: &WriterOption) -> Result<()> {
        writer.write_all(&[*self as u8])
    }
}

impl BinWrite for u8 {
    fn write_options<W: Write>(&self, writer: &mut W, _options: &WriterOption) -> Result<()> {
        writer.write_all(&[*self])
    }
}

impl BinWrite for i8 {
    fn write_options<W: Write>(&self, writer: &mut W, _options: &WriterOption) -> Result<()> {
        writer.write_all(&[*self as u8])
    }
}

binwrite_impl!(
    (u16, write_u16),
    (u32, write_u32),
    (u64, write_u64),
    (i16, write_i16),
    (i32, write_i32),
    (i64, write_i64),
    (f32, write_f32),
    (f64, write_f64),
);

impl<B: BinWrite> BinWrite for Vec<B> {
    fn write_options<W: Write>(&self, writer: &mut W, options: &WriterOption) -> Result<()> {
        for item in self {
            BinWrite::write_options(item, writer, options)?;
        }
        Ok(())
    }
}

impl<B: BinWrite> BinWrite for [B] {
    fn write_options<W: Write>(&self, writer: &mut W, options: &WriterOption) -> Result<()> {
        for item in &self[..] {
            BinWrite::write_options(item, writer, options)?;
        }
        Ok(())
    }
}

macro_rules! binwrite_array_impl {
    ($($size:literal),*$(,)?) => {
        $(
            impl<B: BinWrite> BinWrite for [B; $size] {
                fn write_options<W: Write>(&self, writer: &mut W, options: &WriterOption) -> Result<()> {
                    for item in &self[..] {
                        BinWrite::write_options(item, writer, options)?;
                    }
                    Ok(())
                }
            }
        )*
    }
}

binwrite_array_impl!(0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20);

impl BinWrite for &[u8] {
    fn write_options<W: Write>(&self, writer: &mut W, _options: &WriterOption) -> Result<()> {
        writer.write_all(self)
    }
}

impl BinWrite for str {
    fn write_options<W: Write>(&self, writer: &mut W, _options: &WriterOption) -> Result<()> {
        writer.write_all(self.as_bytes())
    }
}

impl BinWrite for &str {
    fn write_options<W: Write>(&self, writer: &mut W, _options: &WriterOption) -> Result<()> {
        writer.write_all((*self).as_bytes())
    }
}

impl<B: BinWrite> BinWrite for &B {
    fn write_options<W: Write>(&self, writer: &mut W, options: &WriterOption) -> Result<()> {
        (*self).write_options(writer, options)
    }
}

impl BinWrite for String {
    fn write_options<W: Write>(&self, writer: &mut W, options: &WriterOption) -> Result<()> {
        BinWrite::write_options(&self[..], writer, options)
    }
}

/// Internal macro to recursively implement BinWrite for every size tuple 0 to 20
macro_rules! binwrite_tuple_impl {
    ($type1:ident $(, $types:ident)*) => {
        #[allow(non_camel_case_types)]
        impl<$type1: BinWrite, $($types: BinWrite),*> BinWrite for ($type1, $($types),*) {
            paste::item!{
                fn write_options<W: Write>(&self, writer: &mut W, options: &WriterOption) -> Result<()> {
                        let (_, $([<item_ $types>]),*) = self;
                        BinWrite::write_options(&self.0, writer, options)?;
                        $(
                            BinWrite::write_options([<item_ $types>], writer, options)?;
                        )*
                    Ok(())
                }
            }
        }

        binwrite_tuple_impl!($($types),*);
    };

    () => {
        impl BinWrite for () {
            fn write_options<W: Write>(&self, _: &mut W, _: &WriterOption) -> Result<()> {
                Ok(())
            }
        }
    };
}

binwrite_tuple_impl!(b1, b2, b3, b4, b5, b6, b7, b8, b9, b10, b11, b12, b13, b14, b15, b16, b17, b18, b19, b20);

