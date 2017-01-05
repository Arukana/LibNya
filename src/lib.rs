#![crate_type = "dylib"]

#![feature(untagged_unions)]
#![feature(lang_items, libc)]
#![no_std]

extern crate libc;

mod ffi;

use ffi::*;

#[no_mangle]
pub unsafe extern "C" fn start(state: *mut LibraryState, _: *mut *mut libc::c_void) {
    if let Some(mut state) = state.as_mut() {
        state.sheet = Sheet::Bust;
        state.emotion[0][0] = Tuple {
            part: Part::Heart,
            emotion: Emotion::Shocked,
        };
        state.position.cardinal = Cardinal::MiddleCentral;
    }
}

#[lang = "eh_personality"] extern fn rust_eh_personality() {}
#[lang = "panic_fmt"] extern fn rust_begin_panic() -> ! { loop {} }
