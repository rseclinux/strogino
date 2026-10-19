use {
  crate::{support::ffi::va_list::ExtVaList, types::c_longdouble},
  core::ffi::c_void
};

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct ExtVaListInner {
  gpr: i64,
  fpr: i64,
  overflow_arg_area: *const c_void,
  reg_save_area: *const c_void
}

#[inline]
pub unsafe fn get_long_double_value(args: &mut ExtVaList) -> c_longdouble {
  // SINCE ON ELF S390X LONG DOUBLES ARE PASSED AS REFERENCES
  // ON GPRS YOU CAN LITERALLY DO THE FOLLOWING
  // if GPRs get exhausted then off the stack
  let ptr = unsafe { args.next_arg::<*const [u8; 16]>() };
  let result = unsafe { ptr.read() };
  c_longdouble::from_be_bytes(result)
}
