#![crate_type = "dylib"]

#![feature(lang_items, libc)]
#![no_std]

extern crate libc;

/// The limit of draws by sprite.
pub const SPEC_MAX_DRAW: usize = 16;

pub const SPEC_MAX_X: usize = 10;
pub const SPEC_MAX_Y: usize = 5;
pub const SPEC_MAX_XY: usize = SPEC_MAX_X * SPEC_MAX_Y;
pub const SPEC_MAX_PRE_XY: usize = SPEC_MAX_XY - 1;

#[repr(u8)]
pub enum Position {
    UpperLeft = 0,
    UpperMiddle = 1,
    UpperRight = 2,
    MiddleLeft = 3,
    MiddleCentral = 4,
    MiddleRight = 5,
    LowerLeft = 6,
    LowerMiddle = 7,
    LowerRight = 8,
}

#[repr(u8)]
pub enum Sheet {
    None = b'_',
    Bust = b'b',
}

#[repr(C)]
pub struct Tuple {
    pub part: Part,
    pub emotion: Emotion,
}

#[repr(C)]
pub struct LibraryState {
    pub sheet: Sheet,
    pub emotions: [Emotion; SPEC_MAX_DRAW],
    pub draws: [[Tuple; SPEC_MAX_XY]; SPEC_MAX_DRAW],
    pub position: Position,
    pub cartesian: [libc::c_ushort; 2],
    pub message: [libc::c_uchar; 1024],
    pub unmount: libc::c_uchar,
}

#[repr(u8)]
pub enum Part {
    None = b'_',
    ArmLeft = b'a',
    ArmRight = b'A',
    Boobs = b'b',
    Clavicle = b'c',
    EarLeft = b'e',
    EarRight = b'E',
    EyeLeft = b'y',
    EyeRight = b'Y',
    HairTop = b'o',
    HairLeft = b'r',
    HairRight = b'R',
    HandLeft = b'd',
    HandRight = b'D',
    Mouth = b'm',
    Tail = b't',
    Bell = b'l',
    ExclamationMark = b'x',
    ExclamationMarks = b'X',
    Heart = b'h',
    Hearts = b'H',
    Lantern = b'n',
    QuestionMark = b'q',
    QuestionMarks = b'Q',
    WoolBall = b'w',
}

#[repr(u8)]
pub enum Emotion {
    None = b'_',
    Angry = b'a',
    Happy = b'h',
    Love = b'l',
    Malicious = b'm',
    Misunderstanding = b'i',
    Shocked = b'o',
    Sleepy = b's',
    Speechless = b'e',
}

#[no_mangle]
pub unsafe extern "C" fn start(state: *mut LibraryState, _: *mut libc::c_void) {
    if let Some(mut state) = state.as_mut() {
        state.sheet = Sheet::Bust;
        libc::memcpy(
            state.message.as_mut_ptr() as *mut libc::c_void,
            b"hello".as_ptr() as *const libc::c_void,
            5
        );
    }
    libc::write(1, b"hell5\n".as_ptr() as *const libc::c_void, 6);
}

#[lang = "eh_personality"] extern fn rust_eh_personality() {}
#[lang = "panic_fmt"] extern fn rust_begin_panic() -> ! { loop {} }
