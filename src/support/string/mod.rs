use {
  crate::{
    allocation::{borrow::Cow, vec::Vec},
    c_char,
    c_int,
    size_t,
    std::{errno, string}
  },
  core::{fmt, slice, str}
};

pub struct StringStream<'a> {
  data: &'a mut [c_char],
  writeptr: size_t,
  err: bool
}

impl<'a> StringStream<'a> {
  fn write(
    &mut self,
    bytes: *const c_char,
    size: size_t
  ) {
    let b = unsafe { slice::from_raw_parts(bytes, size) };
    let ds = self.data.len();
    let mut i = 0;
    while self.writeptr < ds && i < size {
      self.data[self.writeptr] = b[i];
      i += 1;
      self.writeptr += 1;
    }
    if i < size {
      self.err = true;
    }
  }

  pub fn new(buf: &'a mut [c_char]) -> Self {
    Self { data: &mut buf[..], writeptr: 0, err: false }
  }

  pub fn from_cchar(
    &mut self,
    c: c_char
  ) {
    self.write(&c, 1);
  }

  pub fn from_cstr(
    &mut self,
    cstr: *const c_char
  ) {
    self.write(cstr, string::rs_strlen(cstr));
  }

  pub fn from_str(
    &mut self,
    s: &str
  ) {
    self.write(s.as_ptr().cast(), s.len());
  }

  pub fn as_str(&mut self) -> Result<&str, c_int> {
    let s = unsafe {
      slice::from_raw_parts(self.data.as_ptr().cast::<u8>(), self.data.len())
    };
    match str::from_utf8(s) {
      | Ok(s) => Ok(s),
      | Err(_) => Err(errno::EILSEQ)
    }
  }

  pub fn has_overflow(&self) -> bool {
    self.err
  }
}

impl<'a> fmt::Write for StringStream<'a> {
  fn write_str(
    &mut self,
    s: &str
  ) -> fmt::Result {
    self.from_str(s);
    Ok(())
  }
}

pub fn str_to_cstr<'a>(s: &str) -> Cow<'a, [u8]> {
  let mut result = Vec::new();
  result.extend_from_slice(s.as_bytes());
  result.push(b'\0');
  Cow::Owned(result)
}

pub fn bytestr_to_cstr<'a>(s: &[u8]) -> Cow<'a, [u8]> {
  let mut result = Vec::new();
  result.extend_from_slice(s);
  result.push(b'\0');
  Cow::Owned(result)
}

pub fn bytestr_nul_to_str(s: &[u8]) -> Option<&str> {
  let len_without_nul = s.len() - 1;

  if s[len_without_nul] as u8 != b'\0' {
    return None;
  }

  let conv = &s[..len_without_nul];

  unsafe { Some(str::from_utf8_unchecked(conv)) }
}
