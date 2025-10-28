pub mod ext;

use {
  crate::{
    c_char,
    c_int,
    c_uchar,
    locale_t,
    size_t,
    support::{algorithm::twoway, locale}
  },
  cbitset::BitSet256,
  core::{cmp::Ordering, ffi::c_void, ptr, slice}
};

#[unsafe(no_mangle)]
pub extern "C" fn rs_memccpy(
  dest: *mut c_void,
  src: *const c_void,
  c: c_int,
  n: size_t
) -> *mut c_void {
  let mut d: *mut c_uchar = dest as *mut c_uchar;
  let mut s: *const c_uchar = src as *const c_uchar;
  let mut i = n;
  while i != 0 {
    unsafe {
      *d = *s;
      d = d.offset(1);
      s = s.offset(1);
      if *d.offset(-1) == c as c_uchar {
        return d as *mut c_void;
      }
    }
    i -= 1;
  }
  ptr::null_mut()
}

#[unsafe(no_mangle)]
pub extern "C" fn rs_memchr(
  s: *const c_void,
  c: c_int,
  n: size_t
) -> *mut c_void {
  let mut s1: *const c_uchar = s as *const c_uchar;
  let mut i = n;
  while i != 0 {
    unsafe {
      if *s1 == c as c_uchar {
        return s1 as *mut c_void;
      }
      s1 = s1.offset(1);
    }
    i -= 1;
  }
  ptr::null_mut()
}

#[unsafe(no_mangle)]
pub extern "C" fn rs_memcmp(
  left: *const c_void,
  right: *const c_void,
  n: size_t
) -> c_int {
  let l = left as *const c_uchar;
  let r = right as *const c_uchar;
  let mut i = 0;
  while i < n {
    let a = unsafe { *l.offset(i as isize) };
    let b = unsafe { *r.offset(i as isize) };
    if a != b {
      return a as c_int - b as c_int;
    }
    i += 1;
  }
  0
}

#[unsafe(no_mangle)]
pub extern "C" fn rs_memcpy(
  dest: *mut c_void,
  src: *const c_void,
  n: size_t
) -> *mut c_void {
  let mut d: *mut c_uchar = dest as *mut c_uchar;
  let mut s: *const c_uchar = src as *const c_uchar;
  let mut i = 0;
  while i < n {
    unsafe {
      *d = *s;
      d = d.offset(1);
      s = s.offset(1);
    }
    i += 1;
  }
  dest
}

#[unsafe(no_mangle)]
pub extern "C" fn rs_memmove(
  dest: *mut c_void,
  src: *const c_void,
  n: size_t
) -> *mut c_void {
  let mut d: *mut c_uchar = dest as *mut c_uchar;
  let mut s: *const c_uchar = src as *const c_uchar;
  if (d as *const c_uchar) < s {
    let mut i = 0;
    while i < n {
      unsafe {
        *d = *s;
        d = d.offset(1);
        s = s.offset(1);
      }
      i += 1;
    }
  } else if (d as *const c_uchar) > s {
    let mut i = n;
    unsafe {
      s = s.offset(i as isize);
      d = d.offset(i as isize);

      while i != 0 {
        d = d.offset(-1);
        s = s.offset(-1);
        *d = *s;
        i -= 1;
      }
    }
  }
  dest
}

#[unsafe(no_mangle)]
pub extern "C" fn rs_memset(
  s: *mut c_void,
  c: c_int,
  n: size_t
) -> *mut c_void {
  let mut s1: *mut c_char = s as *mut c_char;
  let mut i = 0;
  while i < n {
    unsafe {
      *s1 = c as c_char;
      s1 = s1.offset(1);
    }
    i += 1;
  }
  s
}

#[unsafe(no_mangle)]
pub extern "C" fn rs_memset_explicit(
  s: *mut c_void,
  c: c_int,
  n: size_t
) -> *mut c_void {
  rs_memset(s, c, n);
  core::sync::atomic::compiler_fence(core::sync::atomic::Ordering::SeqCst);
  s
}

#[unsafe(no_mangle)]
pub extern "C" fn rs_stpcpy(
  dest: *mut c_char,
  src: *const c_char
) -> *mut c_char {
  rs_stpncpy(dest, src, rs_strlen(src) + 1)
}

#[unsafe(no_mangle)]
pub extern "C" fn rs_stpncpy(
  dest: *mut c_char,
  src: *const c_char,
  n: size_t
) -> *mut c_char {
  let mut i = n;
  let mut d = dest;
  let mut s = src;
  unsafe {
    while i > 0 && *s != 0 {
      *d = *s;
      d = d.offset(1);
      s = s.offset(1);
      i -= 1;
    }
  }
  let end = d;
  while i > 0 {
    unsafe {
      *d = 0;
      d = d.offset(1);
    }
    i -= 1;
  }
  end
}

#[unsafe(no_mangle)]
pub extern "C" fn rs_strcat(
  dest: *mut c_char,
  src: *const c_char
) -> *mut c_char {
  rs_strncat(dest, src, rs_strlen(src) + 1)
}

#[unsafe(no_mangle)]
pub extern "C" fn rs_strchr(
  s: *const c_char,
  c: c_int
) -> *mut c_char {
  let mut s1 = s;
  loop {
    unsafe {
      if *s1 == c as c_char {
        return s1 as *mut c_char;
      }
      if *s1 == 0 {
        return ptr::null_mut();
      }
      s1 = s1.offset(1);
    }
  }
}

#[unsafe(no_mangle)]
pub extern "C" fn rs_strcmp(
  left: *const c_char,
  right: *const c_char
) -> c_int {
  rs_strncmp(left, right, usize::MAX)
}

#[unsafe(no_mangle)]
pub extern "C" fn rs_strcoll(
  lhs: *const c_char,
  rhs: *const c_char
) -> c_int {
  rs_strcoll_l(lhs, rhs, locale::get_thread_locale_ptr())
}

#[unsafe(no_mangle)]
pub extern "C" fn rs_strcoll_l(
  lhs: *const c_char,
  rhs: *const c_char,
  locale: locale_t<'static>
) -> c_int {
  let locale = locale::get_real_locale(locale);
  let collate = locale::get_slot(&locale.collate);

  let lhs: &[u8] =
    unsafe { slice::from_raw_parts(lhs as *const u8, rs_strlen(lhs)) };
  let rhs: &[u8] =
    unsafe { slice::from_raw_parts(rhs as *const u8, rs_strlen(rhs)) };

  match collate.collate_u8(lhs, rhs) {
    | Ordering::Less => -1,
    | Ordering::Equal => 0,
    | Ordering::Greater => 1
  }
}

#[unsafe(no_mangle)]
pub extern "C" fn rs_strcpy(
  dest: *mut c_char,
  src: *const c_char
) -> *mut c_char {
  rs_strncpy(dest, src, rs_strlen(src) + 1)
}

#[unsafe(no_mangle)]
pub extern "C" fn rs_strcspn(
  src: *const c_char,
  segment: *const c_char
) -> size_t {
  let mut s1 = src;
  let mut s2 = segment;
  let mut bitset = BitSet256::new();
  let mut i = 0;
  unsafe {
    while *s2 != 0 {
      bitset.insert(*s2 as usize);
      s2 = s2.offset(1);
    }
    while *s1 != 0 && !bitset.contains(*s1 as usize) {
      i += 1;
      s1 = s1.offset(1);
    }
  }
  i
}

#[unsafe(no_mangle)]
pub extern "C" fn rs_strlen(s: *const c_char) -> size_t {
  let mut len: size_t = 0;
  let mut s = s;
  unsafe {
    while *s != 0 {
      s = s.offset(1);
      len += 1;
    }
  }
  len
}

#[unsafe(no_mangle)]
pub extern "C" fn rs_strncat(
  dest: *mut c_char,
  src: *const c_char,
  n: size_t
) -> *mut c_char {
  let mut i = n;

  if n != 0 {
    let mut d = dest;
    let mut s = src;
    unsafe {
      while *d != 0 {
        d = d.offset(1);
      }
    }
    while i != 0 {
      unsafe {
        *d = *s;
        s = s.offset(1);
        if *d == 0 {
          break;
        }
        d = d.offset(1);
      }
      i -= 1;
    }
    unsafe { *d = 0 };
  }
  dest
}

#[unsafe(no_mangle)]
pub extern "C" fn rs_strncmp(
  left: *const c_char,
  right: *const c_char,
  n: size_t
) -> c_int {
  let mut l = left;
  let mut r = right;
  let mut i = n;
  while i != 0 {
    unsafe {
      let c1 = *l as c_uchar;
      l = l.offset(1);
      let c2 = *r as c_uchar;
      r = r.offset(1);
      if c1 != c2 {
        return c1 as c_int - c2 as c_int;
      }
      if c1 == 0 {
        break;
      }
    }
    i -= 1;
  }
  0
}

#[unsafe(no_mangle)]
pub extern "C" fn rs_strncpy(
  dest: *mut c_char,
  src: *const c_char,
  n: size_t
) -> *mut c_char {
  let mut i = n;
  let mut d = dest;
  let mut s = src;
  unsafe {
    while i > 0 && *s != 0 {
      *d = *s;
      d = d.offset(1);
      s = s.offset(1);
      i -= 1;
    }
  }
  while i != 0 {
    unsafe {
      *d = 0;
      d = d.offset(1);
    }
    i -= 1;
  }
  dest
}

#[unsafe(no_mangle)]
pub extern "C" fn rs_strnlen(
  s: *const c_char,
  n: size_t
) -> size_t {
  let mut i = 0;
  while i < n {
    unsafe {
      if *s.add(i) == 0 {
        break;
      }
    }
    i += 1;
  }
  i as size_t
}

#[unsafe(no_mangle)]
pub extern "C" fn rs_strpbrk(
  src: *const c_char,
  breakset: *const c_char
) -> *mut c_char {
  let mut s1 = src;
  let mut s2 = breakset;
  let mut bitset = BitSet256::new();
  unsafe {
    while *s2 != 0 {
      bitset.insert(*s2 as usize);
      s2 = s2.offset(1);
    }
    while *s1 != 0 && !bitset.contains(*s1 as usize) {
      s1 = s1.offset(1);
    }
    if *s1 != 0 { s1 as *mut c_char } else { ptr::null_mut() }
  }
}

#[unsafe(no_mangle)]
pub extern "C" fn rs_strrchr(
  s: *const c_char,
  c: c_int
) -> *mut c_char {
  let mut s1 = s;
  let mut last = ptr::null_mut();
  loop {
    unsafe {
      if *s1 == c as c_char {
        last = s1 as *mut c_char;
      }
      if *s1 == 0 {
        return last;
      }
      s1 = s1.offset(1);
    }
  }
}

#[unsafe(no_mangle)]
pub extern "C" fn rs_strspn(
  src: *const c_char,
  segment: *const c_char
) -> size_t {
  let mut s1 = src;
  let mut s2 = segment;
  let mut bitset = BitSet256::new();
  let mut i = 0;
  unsafe {
    while *s2 != 0 {
      bitset.insert(*s2 as usize);
      s2 = s2.offset(1);
    }
    while *s1 != 0 && bitset.contains(*s1 as usize) {
      i += 1;
      s1 = s1.offset(1);
    }
  }
  i
}

#[unsafe(no_mangle)]
pub extern "C" fn rs_strstr(
  haystack: *const c_char,
  needle: *const c_char
) -> *mut c_char {
  let nlen = rs_strlen(needle);

  if nlen == 0 {
    return haystack.cast_mut();
  }
  if nlen == 1 {
    unsafe { return rs_strchr(haystack, *needle as c_int) };
  }

  let hlen = rs_strlen(haystack);
  let h = unsafe { slice::from_raw_parts(haystack, hlen) };
  let n = unsafe { slice::from_raw_parts(needle, nlen) };
  match twoway::twoway(h, n) {
    | Some(x) => x.as_ptr().cast_mut(),
    | None => ptr::null_mut()
  }
}

#[unsafe(no_mangle)]
pub extern "C" fn rs_strtok(
  s: *mut c_char,
  sep: *const c_char
) -> *mut c_char {
  static mut LAST: *mut c_char = ptr::null_mut();
  rs_strtok_r(s, sep, ptr::addr_of_mut!(LAST))
}

#[unsafe(no_mangle)]
pub extern "C" fn rs_strtok_r(
  s: *mut c_char,
  sep: *const c_char,
  lasts: *mut *mut c_char
) -> *mut c_char {
  let mut s1 = s;
  let mut sep1 = sep;
  let mut bitset = BitSet256::new();
  if s1.is_null() {
    s1 = unsafe { *lasts };
    if s1.is_null() {
      return ptr::null_mut();
    }
  }
  unsafe {
    while *sep1 != 0 {
      bitset.insert(*sep1 as usize);
      sep1 = sep1.offset(1);
    }
    while *s1 != 0 && bitset.contains(*s1 as usize) {
      s1 = s1.offset(1);
    }
    if *s1 == 0 {
      *lasts = s1;
      return ptr::null_mut();
    }
  }
  let token = s1;
  unsafe {
    while *s1 != 0 {
      if bitset.contains(*s1 as usize) {
        *s1 = 0;
        s1 = s1.offset(1);
        break;
      }
      s1 = s1.offset(1);
    }
    *lasts = s1;
  }
  token
}

#[unsafe(no_mangle)]
pub extern "C" fn rs_strxfrm(
  dest: *mut c_char,
  src: *const c_char,
  n: size_t
) -> size_t {
  rs_strxfrm_l(dest, src, n, locale::get_thread_locale_ptr())
}

#[unsafe(no_mangle)]
pub extern "C" fn rs_strxfrm_l(
  dest: *mut c_char,
  src: *const c_char,
  n: size_t,
  locale: locale_t<'static>
) -> size_t {
  let locale = locale::get_real_locale(locale);
  let collate = locale::get_slot(&locale.collate);

  let source: &[u8] =
    unsafe { slice::from_raw_parts(src as *const u8, rs_strlen(src)) };
  let sortkey: &[u8] = &collate.get_sortkey_u8(source);

  if sortkey.len() < n && !sortkey.is_empty() {
    let destination: &mut [u8] =
      unsafe { slice::from_raw_parts_mut(dest as *mut u8, n) };

    destination[..sortkey.len()].copy_from_slice(sortkey);

    destination[sortkey.len()] = b'\0';
  }

  sortkey.len()
}

/*
fn build_error_string(
  num: c_int,
  buf: *mut c_char,
  len: usize,
  locale: LocaleStruct
) -> *const c_char {
  let messages = locale.messages.expect("Malformed locale data");

  let mut ss = unsafe {
    crate::support::string::StringStream::new(slice::from_raw_parts_mut(
      buf, len
    ))
  };
  fmt::write(&mut ss, format_args!("{} {num}\0", messages.unknown_error))
    .expect(
      "Error occurred while trying to write in stream, is buffer too short?"
    );
  buf
}

static STRERROR_BUF: Lazy<[u8; 255]> = Lazy::new(|| [0; 255]);

fn inner_strerror(
  num: c_int,
  buf: *mut c_char,
  len: size_t,
  locale: LocaleStruct
) -> c_int {
  let messages = locale.messages.expect("Malformed locale data");

  if 0 <= num && (num as usize) < messages.strerror.len() {
    let errstr = match messages.strerror.get(num as usize) {
      | Some(x) => x,
      | None => messages.unknown_error
    };
    if (errstr.len() + 1 > len) || buf.is_null() {
      return errno::ERANGE;
    }
    let mut ss = unsafe {
      crate::support::string::StringStream::new(slice::from_raw_parts_mut(
        buf, len
      ))
    };
    fmt::write(&mut ss, format_args!("{errstr}\0")).expect(
      "Error occurred while trying to write in stream, is buffer too short?"
    );
  } else {
    build_error_string(num, buf, len, locale);
    return errno::EINVAL;
  }
  0
}

#[unsafe(no_mangle)]
pub extern "C" fn rs_strerror_r(
  num: c_int,
  buf: *mut c_char,
  len: size_t
) -> c_int {
  inner_strerror(num, buf, len, locale::get_thread_locale())
}

#[unsafe(no_mangle)]
pub extern "C" fn rs_strerror(num: c_int) -> *mut c_char {
  rs_strerror_l(num, &mut locale::get_thread_locale() as locale_t)
}
*/

// do strsignal

// Allocated memory stuff: strdup, strndup
