extern crate libc;

use libc::{c_int};
use std::char;
use std::vec::Vec;

use super::*;

#[derive(Debug, Default, PartialEq)]
pub struct Color {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

#[derive(Debug, Default)]
pub struct ScreenCellAttr {
    pub bold:       bool,
    pub underline:  u8, // 0 to 3
    pub italic:     bool,
    pub blink:      bool,
    pub reverse:    bool,
    pub strike:     bool,
    pub font:       u8, // 0 to 9
    pub dwl:        bool, // On a DECDWL or DECDHL line
    pub dhl:        u8, // On a DECDHL line (1=top 2=bottom)
}

#[derive(Debug, Default)]
pub struct ScreenCell {
    pub chars: Vec<char>,
    pub width: u8,
    pub attrs: ScreenCellAttr,
    pub fg: Color,
    pub bg: Color,
}

impl ScreenCell {
    // Copies data from the given pointer. Doesn't free the pointer or anything.
    pub fn from_ptr(ptr: *const ffi::VTermScreenCell) -> ScreenCell {
        let fg = unsafe { ffi::vterm_cell_get_fg(ptr) };
        let bg = unsafe { ffi::vterm_cell_get_fg(ptr) };

        let mut buf = [0 as libc::uint32_t; ffi::VTERM_MAX_CHARS_PER_CELL];
        let chars_count = unsafe { ffi::vterm_cell_get_chars(ptr, buf.as_mut_ptr(), ffi::VTERM_MAX_CHARS_PER_CELL as u64) };

        let mut chars: Vec<char> = Vec::with_capacity(ffi::VTERM_MAX_CHARS_PER_CELL);

        for i in 0..(chars_count as usize) {
            let ch = char::from_u32(buf[i]).unwrap();
            chars.push(ch);
        }

        unsafe {
            ScreenCell {
                chars: chars,
                width: ffi::vterm_cell_get_width(ptr) as u8,
                attrs: ScreenCellAttr {
                    bold:       int_to_bool(ffi::vterm_cell_get_bold(ptr) as i32),
                    underline:  ffi::vterm_cell_get_underline(ptr) as u8,
                    italic:     int_to_bool(ffi::vterm_cell_get_italic(ptr) as i32),
                    blink:      int_to_bool(ffi::vterm_cell_get_blink(ptr) as i32),
                    reverse:    int_to_bool(ffi::vterm_cell_get_reverse(ptr) as i32),
                    strike:     int_to_bool(ffi::vterm_cell_get_strike(ptr) as i32),
                    font:       ffi::vterm_cell_get_font(ptr) as u8,
                    dwl:        int_to_bool(ffi::vterm_cell_get_dwl(ptr) as i32),
                    dhl:        ffi::vterm_cell_get_dhl(ptr) as u8,
                },
                fg: Color {
                    red: fg.red,
                    green: fg.green,
                    blue: fg.blue,
                },
                bg: Color {
                    red: bg.red,
                    green: bg.green,
                    blue: bg.blue,
                }
            }
        }
    }

    pub fn chars_as_utf8_bytes(&self) -> Vec<u8> {
        const MAX_BYTES_PER_UTF8_CHAR: usize = 4; // wikipedia says so
        let mut output: Vec<u8> = Vec::with_capacity(ffi::VTERM_MAX_CHARS_PER_CELL * MAX_BYTES_PER_UTF8_CHAR);

        for ch in &self.chars {
            let mut bytes = [0 as u8; MAX_BYTES_PER_UTF8_CHAR];
            match ch.encode_utf8(&mut bytes) {
                Some(size) => {
                    for byte in &bytes[0..size] {
                        output.push(*byte);
                    }
                },
                None => panic!("char couldn't be encoded as utf8: {:?}", ch),
            }
        }

        output
    }
}

mod tests {
    use super::super::*;
}
