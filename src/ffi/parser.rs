use libc::{c_int, c_void, c_char, size_t, c_long, c_uchar};

use super::*;

#[repr(C)]
pub struct VTermParserCallbacks {
    pub text: extern fn(bytes: *const c_char, len: size_t, user: *mut c_void) -> c_int,
    pub control: extern fn(control: c_uchar, user: *mut c_void) -> c_int,

    pub escape: extern fn(bytes: *const c_char, lne: size_t, user: *mut c_void) -> c_int,
    pub csi: extern fn(leader: *const c_char, args: *const c_long, argcount: c_int, intermed: *const c_char, command: c_char, user: *mut c_void) -> c_int,
    pub osc: extern fn(command: *const c_char, cmdlen: size_t, user: *mut c_void) -> c_int,
    pub dcs: extern fn(command: *const c_char, cmdlen: size_t, user: *mut c_void) -> c_int,
    pub resize: extern fn(rows: c_int, cols: c_int, user: *mut c_void) -> c_int,
}

extern {
    pub fn vterm_parser_set_callbacks(vt: *mut VTerm, callbacks: *const VTermParserCallbacks, user: *mut c_void) -> *mut VTermState;
    pub fn vterm_parser_get_callbacks(vt: *mut VTerm) -> *mut c_void;
}
