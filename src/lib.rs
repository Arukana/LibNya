#![crate_type = "dylib"]

#![feature(untagged_unions)]
#![feature(lang_items, libc)]
#![no_std]

extern crate libc;

mod ffi;

use ffi::*;

#[no_mangle]
pub unsafe extern "C" fn start(state: *mut LibraryState, _: *mut *mut libc::c_void) {
}

#[lang = "eh_personality"] extern fn rust_eh_personality() {}
#[lang = "panic_fmt"] extern fn rust_begin_panic() -> ! { loop {} }
