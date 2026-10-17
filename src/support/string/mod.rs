pub mod cbuf;
pub mod conversion;
pub mod error;

use {
  allocation::{borrow::Cow, ffi::CString},
  allocator_api2::vec::Vec,
  core::{ffi::CStr, str}
};

#[inline]
pub fn strtocstr(s: &str) -> Cow<'static, CStr> {
  let bytes: Vec<u8> = s.bytes().take_while(|&b| b != 0).collect();

  unsafe { Cow::Owned(CString::from_vec_unchecked(bytes.to_vec())) }
}

#[inline]
pub fn cstrtostr<'a>(cs: &'a CStr) -> Cow<'a, str> {
  cs.to_string_lossy()
}

#[inline]
pub fn strtowcstr(s: &str) -> Cow<'static, [u32]> {
  let mut buf: Vec<u32> = s.chars().into_iter().map(|c| c as u32).collect();

  buf.push('\0' as u32);

  Cow::Owned(buf.to_vec())
}
