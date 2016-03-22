use std::vec::Vec;
use libc::{c_int, c_uint, size_t, uint32_t};

use super::*;

#[derive(Debug, PartialEq, Clone)]
pub struct GlyphInfo {
    pub chars: Vec<u8>,
    pub width: isize,
    pub protected_cell: bool,
    pub dwl: bool, // On a DECDWL or DECDHL line
    pub dhl: u8, // On a DECDHL line (1=top 2=bottom)
}

impl GlyphInfo {
    pub fn from_ptr(ptr: *const ffi::VTermGlyphInfo) -> GlyphInfo {
        let mut buf = [0 as uint32_t; ffi::VTERM_MAX_CHARS_PER_CELL];
        let chars_count = unsafe {
            ffi::vterm_glyph_info_get_chars(ptr,
                                            buf.as_mut_ptr(),
                                            ffi::VTERM_MAX_CHARS_PER_CELL as size_t)
        };
        let buf: [u8; ffi::VTERM_MAX_CHARS_PER_CELL * 4] = unsafe { ::std::mem::transmute(buf) };
        let mut chars: Vec<u8> = vec![];
        chars.extend_from_slice(&buf[0..chars_count as usize * 4]);

        unsafe {
            GlyphInfo {
                chars: chars,
                width: ffi::vterm_glyph_info_width(ptr) as isize,
                protected_cell: int_to_bool(ffi::vterm_glyph_info_width(ptr) as i32),
                dwl: int_to_bool(ffi::vterm_glyph_info_width(ptr)),
                dhl: ffi::vterm_glyph_info_width(ptr) as u8,
            }
        }
    }
}

impl Default for GlyphInfo {
    fn default() -> GlyphInfo {
        GlyphInfo {
            chars: vec![],
            width: 1,
            protected_cell: false,
            dwl: false,
            dhl: 0,
        }
    }
}
