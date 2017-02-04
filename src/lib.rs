#![crate_type = "dylib"]

#![feature(untagged_unions)]
#![feature(lang_items, libc)]
#![no_std]

extern crate libc;

mod ffi;

use ffi::*;
use core::ops::BitAnd;

#[no_mangle]
pub unsafe fn messagecpy(mut start: *mut Character, mut source: *mut libc::c_uchar) {
    let end: *mut Character = (start as usize + SPEC_CHARACTER_MAX) as *mut Character;

    while (*source).ne(&b'\x00').bitand(&(start as libc::c_ulong).lt(&(end as libc::c_ulong))) {
        (*start).glyph = *source as libc::c_uint;
        source = (source as usize + 1) as *mut libc::c_uchar;
        start = (start as usize + 1) as *mut Character;
    }
    while (start as libc::c_ulong).lt(&(end as libc::c_ulong)) {
        (*start).glyph = 0;
        start = (start as usize + 1) as *mut Character;
    }
}

#[no_mangle]
pub unsafe extern "C" fn start(lib: *mut LibraryState, _: *mut *mut libc::c_void) {
    (*lib).persona.sheet = Sheet::BustHappy;
    messagecpy( &mut ((*lib).tooltip.message[0]) as *mut ffi::Character , &b"start\0"[0] as *const libc::c_uchar as *mut libc::c_uchar);
}

#[no_mangle]
pub unsafe extern "C" fn key_unicode_down(lib: *mut LibraryState, _: *mut *mut libc::c_void, key: libc::c_ulonglong) {
    messagecpy( &mut ((*lib).tooltip.message[0]) as *mut ffi::Character , &[key as libc::c_uchar, b'\0'] as *const libc::c_uchar as *mut libc::c_uchar);
}

#[lang = "eh_personality"]
extern "C" fn rust_eh_personality() {}
#[lang = "panic_fmt"]
extern "C" fn rust_begin_panic() -> ! {
    loop {}
}
