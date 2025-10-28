pub mod constants;
pub mod ext;
//pub mod mbstate;

use {
  crate::{
    c_int,
    locale_t,
    size_t,
    support::{algorithm::twoway, locale},
    wchar_t
  },
  cbitset::BitSet256,
  core::{cmp::Ordering, ptr, slice}
};

#[unsafe(no_mangle)]
pub extern "C" fn rs_wmemccpy(
  dest: *mut wchar_t,
  src: *const wchar_t,
  c: wchar_t,
  n: size_t
) -> *mut wchar_t {
  let mut d = dest;
  let mut s = src;
  let mut i = n;
  while i != 0 {
    unsafe {
      *d = *s;
      d = d.offset(1);
      s = s.offset(1);
      if *d.offset(-1) == c {
        return d as *mut wchar_t;
      }
    }
    i -= 1;
  }
  ptr::null_mut()
}

#[unsafe(no_mangle)]
pub extern "C" fn rs_wmemchr(
  s: *const wchar_t,
  c: wchar_t,
  n: size_t
) -> *mut wchar_t {
  let mut s1 = s;
  let mut i = n;
  while i != 0 {
    unsafe {
      if *s1 == c {
        return s1 as *mut wchar_t;
      }
    }
    s1 = s1.wrapping_offset(1);
    i -= 1;
  }
  ptr::null_mut()
}

#[unsafe(no_mangle)]
pub extern "C" fn rs_wmemcmp(
  left: *const wchar_t,
  right: *const wchar_t,
  n: size_t
) -> c_int {
  let l = left;
  let r = right;
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
pub extern "C" fn rs_wmemcpy(
  dest: *mut wchar_t,
  src: *const wchar_t,
  n: size_t
) -> *mut wchar_t {
  let mut d = dest;
  let mut s = src;
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
pub extern "C" fn rs_wmemmove(
  dest: *mut wchar_t,
  src: *const wchar_t,
  n: size_t
) -> *mut wchar_t {
  let mut d = dest;
  let mut s = src;
  if (d as *const wchar_t) < s {
    let mut i = 0;
    while i < n {
      unsafe {
        *d = *s;
        d = d.offset(1);
        s = s.offset(1);
      }
      i += 1;
    }
  } else if (d as *const wchar_t) > s {
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
pub extern "C" fn rs_wmemset(
  s: *mut wchar_t,
  c: wchar_t,
  n: size_t
) -> *mut wchar_t {
  let mut s1 = s;
  let mut i = 0;
  while i < n {
    unsafe {
      *s1 = c;
      s1 = s1.offset(1);
    }
    i += 1;
  }
  s
}

#[unsafe(no_mangle)]
pub extern "C" fn rs_wcpcpy(
  dest: *mut wchar_t,
  src: *const wchar_t
) -> *mut wchar_t {
  rs_wcpncpy(dest, src, rs_wcslen(src) + 1)
}

#[unsafe(no_mangle)]
pub extern "C" fn rs_wcpncpy(
  dest: *mut wchar_t,
  src: *const wchar_t,
  n: size_t
) -> *mut wchar_t {
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

// do wcscasecmp

#[unsafe(no_mangle)]
pub extern "C" fn rs_wcscat(
  dest: *mut wchar_t,
  src: *const wchar_t
) -> *mut wchar_t {
  rs_wcsncat(dest, src, rs_wcslen(src) + 1)
}

#[unsafe(no_mangle)]
pub extern "C" fn rs_wcschr(
  s: *const wchar_t,
  c: wchar_t
) -> *mut wchar_t {
  let mut s1 = s;
  loop {
    unsafe {
      if *s1 == c {
        return s1 as *mut wchar_t;
      }
      if *s1 == 0 {
        return ptr::null_mut();
      }
      s1 = s1.offset(1);
    }
  }
}

#[unsafe(no_mangle)]
pub extern "C" fn rs_wcscmp(
  left: *const wchar_t,
  right: *const wchar_t
) -> c_int {
  rs_wcsncmp(left, right, usize::MAX)
}

#[unsafe(no_mangle)]
pub extern "C" fn rs_wcscoll(
  lhs: *const wchar_t,
  rhs: *const wchar_t
) -> c_int {
  rs_wcscoll_l(lhs, rhs, locale::get_thread_locale_ptr())
}

#[unsafe(no_mangle)]
pub extern "C" fn rs_wcscoll_l(
  lhs: *const wchar_t,
  rhs: *const wchar_t,
  locale: locale_t<'static>
) -> c_int {
  let locale = locale::get_real_locale(locale);
  let collate = locale::get_slot(&locale.collate);

  let lhs: &[u32] =
    unsafe { slice::from_raw_parts(lhs as *const u32, rs_wcslen(lhs)) };
  let rhs: &[u32] =
    unsafe { slice::from_raw_parts(rhs as *const u32, rs_wcslen(rhs)) };

  match collate.collate_u32(lhs, rhs) {
    | Ordering::Less => -1,
    | Ordering::Equal => 0,
    | Ordering::Greater => 1
  }
}

#[unsafe(no_mangle)]
pub extern "C" fn rs_wcscpy(
  dest: *mut wchar_t,
  src: *const wchar_t
) -> *mut wchar_t {
  rs_wcsncpy(dest, src, rs_wcslen(src) + 1)
}

#[unsafe(no_mangle)]
pub extern "C" fn rs_wcscspn(
  src: *const wchar_t,
  segment: *const wchar_t
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
pub extern "C" fn rs_wcslen(s: *const wchar_t) -> size_t {
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

// do wcsncasecmp

#[unsafe(no_mangle)]
pub extern "C" fn rs_wcsncat(
  dest: *mut wchar_t,
  src: *const wchar_t,
  n: size_t
) -> *mut wchar_t {
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
pub extern "C" fn rs_wcsncmp(
  left: *const wchar_t,
  right: *const wchar_t,
  n: size_t
) -> c_int {
  let mut l = left;
  let mut r = right;
  let mut i = n;
  while i != 0 {
    unsafe {
      let c1 = *l as wchar_t;
      l = l.offset(1);
      let c2 = *r as wchar_t;
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
pub extern "C" fn rs_wcsncpy(
  dest: *mut wchar_t,
  src: *const wchar_t,
  n: size_t
) -> *mut wchar_t {
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
pub extern "C" fn rs_wcsnlen(
  s: *const wchar_t,
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
pub extern "C" fn rs_wcspbrk(
  src: *const wchar_t,
  breakset: *const wchar_t
) -> *mut wchar_t {
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
    if *s1 != 0 { s1 as *mut wchar_t } else { ptr::null_mut() }
  }
}

#[unsafe(no_mangle)]
pub extern "C" fn rs_wcsrchr(
  s: *const wchar_t,
  c: wchar_t
) -> *mut wchar_t {
  let mut s1 = s;
  let mut last = ptr::null_mut();
  loop {
    unsafe {
      if *s1 == c {
        last = s1 as *mut wchar_t;
      }
      if *s1 == 0 {
        return last;
      }
      s1 = s1.offset(1);
    }
  }
}

#[unsafe(no_mangle)]
pub extern "C" fn rs_wcsspn(
  src: *const wchar_t,
  segment: *const wchar_t
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
pub extern "C" fn rs_wcsstr(
  haystack: *const wchar_t,
  needle: *const wchar_t
) -> *mut wchar_t {
  let nlen = rs_wcslen(needle);

  if nlen == 0 {
    return haystack.cast_mut();
  }
  if nlen == 1 {
    unsafe { return rs_wcschr(haystack, *needle as wchar_t) };
  }

  let hlen = rs_wcslen(haystack);
  let h = unsafe { slice::from_raw_parts(haystack, hlen) };
  let n = unsafe { slice::from_raw_parts(needle, nlen) };
  match twoway::twoway(h, n) {
    | Some(x) => x.as_ptr().cast_mut(),
    | None => ptr::null_mut()
  }
}

#[unsafe(no_mangle)]
pub extern "C" fn rs_wcstok(
  s: *mut wchar_t,
  sep: *const wchar_t,
  lasts: *mut *mut wchar_t
) -> *mut wchar_t {
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
pub extern "C" fn rs_wcsxfrm(
  dest: *mut wchar_t,
  src: *const wchar_t,
  n: size_t
) -> size_t {
  rs_wcsxfrm_l(dest, src, n, locale::get_thread_locale_ptr())
}

#[unsafe(no_mangle)]
pub extern "C" fn rs_wcsxfrm_l(
  dest: *mut wchar_t,
  src: *const wchar_t,
  n: size_t,
  locale: locale_t<'static>
) -> size_t {
  let locale = locale::get_real_locale(locale);
  let collate = locale::get_slot(&locale.collate);

  let source: &[u32] =
    unsafe { slice::from_raw_parts(src as *const u32, rs_wcslen(src)) };
  let sortkey: &[u32] = &collate.get_sortkey_u32(source);

  if sortkey.len() < n && !sortkey.is_empty() {
    let destination: &mut [u32] =
      unsafe { slice::from_raw_parts_mut(dest as *mut u32, n) };

    destination[..sortkey.len()].copy_from_slice(sortkey);

    destination[sortkey.len()] = '\0' as u32;
  }

  sortkey.len()
}

// Allocated memory stuff: wcsdup
