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
pub unsafe fn get_long_double_value(args: &mut ExtVaList) -> c_longdouble {
  // On 32-bit ARM EABI long double is just f64
  let v = unsafe { args.next_arg::<f64>() };
  v
}
