use std::char;
use std::vec::Vec;

use super::*;

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Color {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

#[derive(Debug, Default, PartialEq, Clone)]
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

#[derive(Debug, PartialEq, Clone)]
pub struct ScreenCell {
    /// Where the cell is located
    pub pos: Pos,
    /// The characters in the cell. I believe there are more than one to support overstrike.
    pub chars: Vec<char>,
    /// I think this is How wide the cell is in columns.
    pub width: u8,
    pub attrs: ScreenCellAttr,
    /// foreground color
    pub fg: Color,
    /// background color
    pub bg: Color,
}

impl ScreenCell {
    // Copies data from the given pointer. Doesn't free the pointer or anything.
    pub fn from_ptr(ptr: *const ffi::VTermScreenCell, pos: Pos) -> ScreenCell {
        let fg = unsafe { ffi::vterm_cell_get_fg(ptr) };
        let bg = unsafe { ffi::vterm_cell_get_bg(ptr) };

        let mut buf = [0 as libc::uint32_t; ffi::VTERM_MAX_CHARS_PER_CELL];
        let chars_count = unsafe { ffi::vterm_cell_get_chars(ptr, buf.as_mut_ptr(), ffi::VTERM_MAX_CHARS_PER_CELL as libc::size_t) };

        let mut chars: Vec<char> = Vec::with_capacity(ffi::VTERM_MAX_CHARS_PER_CELL);

        for i in 0..(chars_count as usize) {
            let ch = match char::from_u32(buf[i]) {
                Some(ch) => ch,
                None => '\u{2764}',
            };

            chars.push(ch);
        }

        unsafe {
            ScreenCell {
                pos:   pos,
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
                    red:    fg.red,
                    green:  fg.green,
                    blue:   fg.blue,
                },
                bg: Color {
                    red:    bg.red,
                    green:  bg.green,
                    blue:   bg.blue,
                }
            }
        }
    }

    pub fn chars_as_utf8_bytes(&self) -> Vec<u8> {
        const MAX_BYTES_PER_UTF8_CHAR: usize = 6; // wikipedia says so
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

impl Default for ScreenCell {
    fn default() -> ScreenCell {
        ScreenCell {
            pos: Default::default(),
            chars: vec!(),
            width: 1,
            attrs: Default::default(),
            fg: Color { red: 230, green: 230, blue: 230 },
            bg: Color { red: 5, green: 5, blue: 5 },
        }
    }
}
