use {
  crate::types::{c_int, c_long, c_longlong},
  core::ops::Neg
};

#[repr(C)]
#[derive(Clone, Copy)]
pub struct div_t {
  pub quot: c_int,
  pub rem: c_int
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct ldiv_t {
  pub quot: c_long,
  pub rem: c_long
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct lldiv_t {
  pub quot: c_longlong,
  pub rem: c_longlong
}

#[unsafe(no_mangle)]
extern "C" fn rs_div(
  num: c_int,
  denom: c_int
) -> div_t {
  div_t { quot: num / denom, rem: num % denom }
}

#[unsafe(no_mangle)]
extern "C" fn rs_ldiv(
  num: c_long,
  denom: c_long
) -> ldiv_t {
  ldiv_t { quot: num / denom, rem: num % denom }
}

#[unsafe(no_mangle)]
extern "C" fn rs_lldiv(
  num: c_longlong,
  denom: c_longlong
) -> lldiv_t {
  lldiv_t { quot: num / denom, rem: num % denom }
}

#[unsafe(no_mangle)]
extern "C" fn rs_abs(num: c_int) -> c_int {
  return if num < 0 { num.neg() } else { num };
}

#[unsafe(no_mangle)]
extern "C" fn rs_labs(num: c_long) -> c_long {
  return if num < 0 { num.neg() } else { num };
}

#[unsafe(no_mangle)]
extern "C" fn rs_llabs(num: c_longlong) -> c_longlong {
  return if num < 0 { num.neg() } else { num };
}
