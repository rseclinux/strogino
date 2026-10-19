use {
  crate::{support::ffi::va_list::ExtVaList, types::c_longdouble},
  core::ffi::c_void
};

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct ExtVaListInner {
  stack: *const c_void,
  gr_top: *const c_void,
  vr_top: *const c_void,
  gr_offs: i32,
  vr_offs: i32
}

#[inline]
pub unsafe fn get_long_double_value(me: &mut ExtVaList) -> c_longdouble {
  let dst: [u8; 16];

  if !me.inner.vr_top.is_null() {
    // In case if long double is on floating point/simd registers
    let ptr = unsafe {
      (me.inner.vr_top as *const u8).offset(me.inner.vr_offs as isize)
    };
    let aligned = (ptr as usize + 15) & !15; // Align up

    let src = aligned as *const [u8; 16];
    dst = unsafe { src.read() };

    me.inner.vr_offs += 16;
  } else {
    // otherwise it is on stack
    let ptr = me.inner.stack as usize;
    let aligned = (ptr + 15) & !15; // Align up

    let src = aligned as *const [u8; 16];
    dst = unsafe { src.read() };

    me.inner.stack = (aligned + 16) as *const c_void; // advance pointer for futher reading
  }

  c_longdouble::from_ne_bytes(dst)
}
