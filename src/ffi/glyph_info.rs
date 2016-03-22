use libc::{c_int, c_uint, size_t, uint32_t};
use super::*;

pub enum VTermGlyphInfo {}

extern "C" {
    // These are my rust ffi bitfield workarounds
    pub fn vterm_glyph_info_get_chars(glyph_info: *const VTermGlyphInfo,
                                      chars: *mut uint32_t,
                                      len: size_t)
                                      -> c_int;
    pub fn vterm_glyph_info_width(glyph_info: *const VTermGlyphInfo) -> c_int;
    pub fn vterm_glyph_info_protected_cell(glyph_info: *const VTermGlyphInfo) -> c_uint;
    pub fn vterm_glyph_info_dwl(glyph_info: *const VTermGlyphInfo) -> c_uint;
    pub fn vterm_glyph_info_dhl(glyph_info: *const VTermGlyphInfo) -> c_uint;
}
