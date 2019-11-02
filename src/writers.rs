use super::*;

pub fn null_terminated_string<S: std::fmt::Display, W: Write>(string: S, writer: &mut W, options: &WriterOption) -> Result<()> {
    BinWrite::write_options(&format!("{}", string), writer, options)?;
    BinWrite::write_options(&0u8, writer, options)
}

pub fn utf16_string<S: std::fmt::Display, W: Write>(string: S, writer: &mut W, options: &WriterOption) -> Result<()> {
    for c in format!("{}", string)[..].encode_utf16() {
        BinWrite::write_options(&c, writer, options)?;
    }
    Ok(())
}

pub fn utf16_null_string<S: std::fmt::Display, W: Write>(string: S, writer: &mut W, options: &WriterOption) -> Result<()> {
    utf16_string(string, writer, options)?;
    BinWrite::write_options(&0u16, writer, options)
}

