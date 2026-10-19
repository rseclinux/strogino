use {
  super::{error::FormatError, length::LengthModifier},
  crate::support::traits::char::CharToAscii,
  core::ascii
};

pub mod float_format;
pub mod strfrom;

pub trait Emitter {
  type FormatChar: Copy + Into<CharToAscii>;

  fn get_written(&self) -> usize;
  fn emit_u8_slice(
    &mut self,
    s: &[u8]
  ) -> Result<(), FormatError>;
  fn emit_ascii_slice(
    &mut self,
    s: &[ascii::Char]
  ) -> Result<(), FormatError>;
  fn emit_u32_slice(
    &mut self,
    s: &[u32]
  ) -> Result<(), FormatError>;
  fn emit_format_string(
    &mut self,
    s: &[Self::FormatChar]
  ) -> Result<(), FormatError>;
  fn get_unicode_char_len(c: char) -> usize;

  #[inline]
  fn emit_ascii_char(
    &mut self,
    c: ascii::Char
  ) -> Result<(), FormatError> {
    self.emit_ascii_slice(&[c])
  }

  #[inline]
  fn emit_unicode_char(
    &mut self,
    c: char
  ) -> Result<(), FormatError> {
    self.emit_u32_slice(&[c as u32])
  }

  #[inline]
  fn pad_to(
    &mut self,
    arg: &PrintfArgument,
    c: ascii::Char,
    n: usize
  ) -> Result<(), FormatError> {
    let mut width = arg.width;
    while width > n {
      self.emit_ascii_char(c)?;
      width -= 1;
    }
    Ok(())
  }
}

#[derive(Debug, Default, Clone, Copy)]
pub struct PrintfFlags {
  pub alternate_form: bool,
  pub leading_zeroes: bool,
  pub left_align: bool,
  pub space_prefix: bool,
  pub prepend_plus: bool,
  pub group_decimals: bool
}

#[derive(Debug, Default, Clone)]
pub struct PrintfArgument {
  pub flags: PrintfFlags,
  pub width: usize,
  pub precision: Option<u32>,
  pub modifier: LengthModifier,
  pub specifier: char
}
