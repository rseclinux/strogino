pub mod constants;

use crate::{c_int, support::locale};

//#[unsafe(no_mangle)]
//pub extern "C" fn __stroginointernal_get_mb_cur_max() -> c_int {
//  let ctype: locale::ctype::LCCtype = locale::get_thread_locale().ctype;
//  ctype.mb_cur_max
//}
