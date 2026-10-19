use {
  crate::{support::ffi::va_list::ExtVaList, types::c_longdouble},
  core::ffi::c_void
};

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct ExtVaListInner {
  ptr: *const c_void
}

#[inline]
pub unsafe fn get_long_double_value(me: &mut ExtVaList) -> c_longdouble {
  let aligned = (me.inner.ptr as usize + 3) & !3; // align up
  let src = aligned as *const [u8; 12];

  let result = unsafe { src.read() };

  // 12 = 10 bytes (the Intel floating-point number itself) + 2 bytes is alignment
  me.inner.ptr = (aligned + 12) as *const c_void; // advance pointer for futher reading

  c_longdouble::from_le_bytes(result) // 80-bit Intel floating-point value
}
