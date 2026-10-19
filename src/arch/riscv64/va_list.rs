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
  let aligned = (me.inner.ptr as usize + 15) & !15; // align up
  let src = aligned as *const [u8; 16];

  let result = unsafe { src.read() };

  me.inner.ptr = (aligned + 16) as *const c_void; // advance pointer for futher reading

  c_longdouble::from_ne_bytes(result) // IEEE quadruple precision floating-point value
}
