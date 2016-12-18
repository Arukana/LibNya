#![crate_type = "dylib"]

#![feature(untagged_unions)]
#![feature(lang_items, libc)]
#![no_std]

extern crate libc;

mod ffi;

use ffi::*;

#[no_mangle]
pub unsafe extern "C" fn start(state: *mut LibraryState, _: *mut libc::c_void) {
    if let Some(mut state) = state.as_mut() {
        state.sheet = Sheet::Bust;
        state.emotion[0][0] = Tuple {
            part: Part::Heart,
            emotion: Emotion::Shocked,
        };
        state.position.cardinal = Cardinal::MiddleCentral;
        /*
        libc::memcpy(
            state.message.as_mut_ptr() as *mut libc::c_void,
            b"hello".as_ptr() as *const libc::c_void,
            5
        );*/
    }
}

#[lang = "eh_personality"] extern fn rust_eh_personality() {}
#[lang = "panic_fmt"] extern fn rust_begin_panic() -> ! { loop {} }
