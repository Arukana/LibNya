#![crate_type = "dylib"]

#![feature(lang_items, libc)]
#![no_std]

extern crate libc;

#[repr(C)]
pub struct LibraryState {
    pub sheet: libc::c_uchar,
    pub emotions: [libc::c_uchar; 16],
    pub draws: [[libc::c_uchar; 140]; 16],
    pub message: [libc::c_uchar; 1024],
    pub unmount: libc::c_uchar,
}

#[no_mangle]
pub unsafe extern "C" fn start(state: *mut LibraryState, _: *mut libc::c_void) {
    if let Some(mut state) = state.as_mut() {
        state.sheet = b'_';
        state.message.copy_from_slice(&"hello"[..].as_bytes());
    }
}

#[lang = "eh_personality"] extern fn rust_eh_personality() {}
#[lang = "panic_fmt"] extern fn rust_begin_panic() -> ! { loop {} }
