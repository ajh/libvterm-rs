use std::char;
use std::vec::Vec;
use libc::{uint32_t, size_t};

use super::*;

#[derive(Debug, Default, PartialEq, Clone)]
pub struct ColorRGB {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

pub type ColorPalette = usize;

#[derive(Debug, Default, PartialEq, Clone)]
pub struct ScreenCellAttr {
    pub bold: bool,
    pub underline: u8, // 0 to 3
    pub italic: bool,
    pub blink: bool,
    pub reverse: bool,
    pub strike: bool,
    pub font: u8, // 0 to 9
    pub dwl: bool, // On a DECDWL or DECDHL line
    pub dhl: u8, // On a DECDHL line (1=top 2=bottom)
}

#[derive(Debug, PartialEq, Clone)]
pub struct ScreenCell {
    /// The characters in the cell. I believe there are more than one to support overstrike. This
    /// is also madness to represent these as chars. They should be u8s.
    pub chars: Vec<char>,
    /// I think this is How wide the cell is in columns.
    pub width: u8,
    pub attrs: ScreenCellAttr,
    /// foreground color
    pub fg_rgb: ColorRGB,
    /// background color
    pub bg_rgb: ColorRGB,

    pub fg_palette: ColorPalette,
    pub bg_palette: ColorPalette,
}

impl ScreenCell {
    // Copies data from the given pointer. Doesn't free the pointer or anything.
    pub fn from_ptr(ptr: *const ffi::VTermScreenCell, vterm: &VTerm) -> ScreenCell {
        let fg_rgb = unsafe { ffi::vterm_cell_get_fg(ptr) };
        let bg_rgb = unsafe { ffi::vterm_cell_get_bg(ptr) };

        let mut buf = [0 as uint32_t; ffi::VTERM_MAX_CHARS_PER_CELL];
        let chars_count = unsafe {
            ffi::vterm_cell_get_chars(ptr,
                                      buf.as_mut_ptr(),
                                      ffi::VTERM_MAX_CHARS_PER_CELL as size_t)
        };

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
                chars: chars,
                width: ffi::vterm_cell_get_width(ptr) as u8,
                attrs: ScreenCellAttr {
                    bold: int_to_bool(ffi::vterm_cell_get_bold(ptr) as i32),
                    underline: ffi::vterm_cell_get_underline(ptr) as u8,
                    italic: int_to_bool(ffi::vterm_cell_get_italic(ptr) as i32),
                    blink: int_to_bool(ffi::vterm_cell_get_blink(ptr) as i32),
                    reverse: int_to_bool(ffi::vterm_cell_get_reverse(ptr) as i32),
                    strike: int_to_bool(ffi::vterm_cell_get_strike(ptr) as i32),
                    font: ffi::vterm_cell_get_font(ptr) as u8,
                    dwl: int_to_bool(ffi::vterm_cell_get_dwl(ptr) as i32),
                    dhl: ffi::vterm_cell_get_dhl(ptr) as u8,
                },
                fg_rgb: ColorRGB {
                    red: fg_rgb.red,
                    green: fg_rgb.green,
                    blue: fg_rgb.blue,
                },
                bg_rgb: ColorRGB {
                    red: bg_rgb.red,
                    green: bg_rgb.green,
                    blue: bg_rgb.blue,
                },
                fg_palette: vterm.state_get_palette_color_from_c_rgb(&fg_rgb),
                bg_palette: vterm.state_get_palette_color_from_c_rgb(&bg_rgb),
            }
        }
    }

    pub fn chars_as_utf8_bytes(&self) -> Vec<u8> {
        let mut output = String::new();
        for ch in &self.chars {
            output.push(*ch)
        }
        output.into_bytes()
    }
}

impl Default for ScreenCell {
    fn default() -> ScreenCell {
        ScreenCell {
            chars: vec![],
            width: 1,
            attrs: Default::default(),
            fg_rgb: ColorRGB {
                red: 230,
                green: 230,
                blue: 230,
            },
            bg_rgb: ColorRGB {
                red: 5,
                green: 5,
                blue: 5,
            },
            fg_palette: 7,
            bg_palette: 0,
        }
    }
}
