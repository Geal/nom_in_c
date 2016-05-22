#![feature(no_std,core)]
#![no_std]

use core::prelude::*;

extern crate libc;
extern crate embed;
//extern crate core;
use libc::{size_t, c_int, c_char};
use core::slice;

#[no_mangle]
pub fn test() -> c_int {
  embed::it_works()
}

#[no_mangle]
pub unsafe extern "C" fn count_noms(s: *const c_char, length: size_t) -> c_int {
  let sl = slice::from_raw_parts(s as *const u8, length as usize);
  if let Some(len) = embed::count_noms(sl) {
    len as i32
  } else {
    -1
  }
}


