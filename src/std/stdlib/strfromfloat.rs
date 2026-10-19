use {
  super::EncodedLDBLReturn,
  crate::{
    c_char,
    c_int,
    c_longdouble,
    size_t,
    std::{errno, string},
    support::{
      format::printf::{Emitter, strfrom},
      locale
    }
  },
  core::slice
};

#[unsafe(no_mangle)]
extern "C" fn __oumainternal_strfromenc(
  s: *mut c_char,
  n: size_t,
  format: *const c_char,
  val: EncodedLDBLReturn
) -> c_int {
  let format = unsafe {
    slice::from_raw_parts(format as *const u8, string::rs_strlen(format))
  };

  let value = c_longdouble::from_ne_bytes(val.bytes);

  let locale = locale::Locale::new();
  let ctype = locale::get_slot(&locale.ctype).unwrap_or_default();
  let numeric = locale::get_slot(&locale.numeric).unwrap_or_default();

  let mut emitter = strfrom::StrFromEmitter::new(s, n, &ctype);

  let result =
    strfrom::strfromfloat(&mut emitter, format, value, &ctype, &numeric);

  if n > 0 {
    emitter.terminate();
  }

  match result {
    | Ok(_) => emitter.get_written() as c_int,
    | Err(e) => {
      if e.eligible_for_errno() {
        errno::set_errno(e.to_errno());
      }
      -1
    }
  }
}
