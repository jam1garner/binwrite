use super::*;

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
        writer.write(&[*self as u8])?;
        Ok(())
    }
}

impl BinWrite for u8 {
    fn write_options<W: Write>(&self, writer: &mut W, _options: &WriterOption) -> Result<()> {
        writer.write(&[*self])?;
        Ok(())
    }
}

impl BinWrite for i8 {
    fn write_options<W: Write>(&self, writer: &mut W, _options: &WriterOption) -> Result<()> {
        writer.write(&[*self as u8])?;
        Ok(())
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

impl BinWrite for str {
    fn write_options<W: Write>(&self, writer: &mut W, _options: &WriterOption) -> Result<()> {
        writer.write_all(self.as_bytes())
    }
}

impl BinWrite for String {
    fn write_options<W: Write>(&self, writer: &mut W, options: &WriterOption) -> Result<()> {
        BinWrite::write_options(&self[..], writer, options)
    }
}

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

/*macro_rules! binwrite_tuple_impl {
    (($num1:literal: $type1: ident: $name1:ident), $(($nums:literal: $types:ident: $names:ident)),*) => {
        impl<$type1: BinWrite, $($types: BinWrite),*> BinWrite for ($type1, $($types),*) {
            fn write_options<W: Write>(&self, writer: &mut W, options: &WriterOption) -> Result<()> {
                let (_, $($names),*) = self;
                BinWrite::write_options(&self.0, writer, options)?;
                $(
                    BinWrite::write_options($names, writer, options)?;
                )*
                Ok(())
            }
        }
    };

    () => {
        impl BinWrite for () {
            fn write_options<W: Write>(&self, writer: &mut W, options: &WriterOption) -> Result<()> {
                Ok(())
            }
        }
    };

}

binwrite_tuple_impl!((0: B1: b1), (1: B2: b2), (2: B3: b3), (3: B4: b4), (4: B5: b5), (5: B6: b6), (6: B7: b7), (7: B8: b8), (8: B9: b9), (9: B10: b10), (10: B11: b11), (11: B12: b12), (12: B13: b13), (13: B14: b14), (14: B15: b15), (15: B16: b16), (16: B17: b17), (17: B18: b18), (18: B19: b19), (19: B20: b20));
*/
