use std::ffi::CString;
use std::io::Read;
use std::io::{self};
use std::os::raw::c_char;
use std::os::raw::c_uchar;
use std::ptr;

#[unsafe(no_mangle)]
pub extern "C" fn readStdin() -> *mut c_char {
  let mut buffer = Vec::new();
  let stdin = io::stdin();
  let mut handle = stdin.lock();

  if handle.read_to_end(&mut buffer).is_err() {
    return ptr::null_mut();
  }

  match CString::new(buffer) {
    Ok(c_str) => c_str.into_raw(),
    Err(_) => ptr::null_mut(),
  }
}

#[unsafe(no_mangle)]
pub extern "C" fn getByte(
  ptr: *const c_char,
  index: i32,
) -> c_uchar {
  if ptr.is_null() {
    return 0;
  }
  unsafe {
    let val = *ptr.offset(index as isize);
    val as c_uchar
  }
}

#[unsafe(no_mangle)]
pub extern "C" fn freeString(ptr: *mut c_char) {
  if !ptr.is_null() {
    unsafe {
      let _ = CString::from_raw(ptr);
    }
  }
}
