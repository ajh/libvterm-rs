use libc::{c_int, c_uint, uint32_t, size_t, c_char, uint8_t};

pub enum VTermScreenCell {}

/// There should be rust methods here to pull stuff out of the cell, also a into_screen_cell method
/// to cast to ScreenCell

pub const VTERM_MAX_CHARS_PER_CELL: usize = 6;

#[repr(C)]
#[derive(PartialEq, Debug, Clone, Default)]
pub struct VTermColor {
    pub red: uint8_t,
    pub green: uint8_t,
    pub blue: uint8_t,
}

impl VTermColor {
    pub fn as_color_rgb(&self) -> ::ColorRGB {
        ::ColorRGB {
            red: self.red,
            green: self.green,
            blue: self.blue,
        }
    }
}

extern "C" {
    // These are my rust ffi bitfield workarounds
    pub fn vterm_cell_new() -> *mut VTermScreenCell;
    pub fn vterm_cell_free(cell: *mut VTermScreenCell);
    pub fn vterm_cell_get_chars(cell: *const VTermScreenCell,
                                chars: *mut uint32_t,
                                len: size_t)
                                -> c_int;
    pub fn vterm_cell_set_chars(cell: *mut VTermScreenCell, chars: *const uint32_t, len: size_t);
    pub fn vterm_cell_get_width(cell: *const VTermScreenCell) -> c_char;
    pub fn vterm_cell_set_width(cell: *mut VTermScreenCell, width: c_char);
    pub fn vterm_cell_get_bold(cell: *const VTermScreenCell) -> c_uint;
    pub fn vterm_cell_set_bold(cell: *mut VTermScreenCell, is_bold: c_uint);
    pub fn vterm_cell_get_underline(cell: *const VTermScreenCell) -> c_uint;
    pub fn vterm_cell_set_underline(cell: *mut VTermScreenCell, underline_value: c_uint);
    pub fn vterm_cell_get_italic(cell: *const VTermScreenCell) -> c_uint;
    pub fn vterm_cell_set_italic(cell: *mut VTermScreenCell, is_italic: c_uint);
    pub fn vterm_cell_get_blink(cell: *const VTermScreenCell) -> c_uint;
    pub fn vterm_cell_set_blink(cell: *mut VTermScreenCell, is_blink: c_uint);
    pub fn vterm_cell_get_reverse(cell: *const VTermScreenCell) -> c_uint;
    pub fn vterm_cell_set_reverse(cell: *mut VTermScreenCell, is_reverse: c_uint);
    pub fn vterm_cell_get_strike(cell: *const VTermScreenCell) -> c_uint;
    pub fn vterm_cell_set_strike(cell: *mut VTermScreenCell, is_strike: c_uint);
    pub fn vterm_cell_get_font(cell: *const VTermScreenCell) -> c_uint;
    pub fn vterm_cell_set_font(cell: *mut VTermScreenCell, font_value: c_uint);
    pub fn vterm_cell_get_dwl(cell: *const VTermScreenCell) -> c_uint;
    pub fn vterm_cell_set_dwl(cell: *mut VTermScreenCell, dwl: c_uint);
    pub fn vterm_cell_get_dhl(cell: *const VTermScreenCell) -> c_uint;
    pub fn vterm_cell_set_dhl(cell: *mut VTermScreenCell, dhl: c_uint);
    pub fn vterm_cell_get_fg(cell: *const VTermScreenCell) -> VTermColor;
    pub fn vterm_cell_set_fg(cell: *mut VTermScreenCell, color: VTermColor);
    pub fn vterm_cell_get_bg(cell: *const VTermScreenCell) -> VTermColor;
    pub fn vterm_cell_set_bg(cell: *mut VTermScreenCell, color: VTermColor);
    pub fn vterm_cell_pointer_arithmetic(cell: *const VTermScreenCell,
                                         amount: c_int)
                                         -> *const VTermScreenCell;
}

mod tests {
    #![allow(unused_imports)]
    use libc::size_t;
    use super::super::*;

    #[test]
    fn ffi_cell_can_create_and_destroy() {
        unsafe {
            let cell_ptr: *mut VTermScreenCell = vterm_cell_new();
            vterm_cell_free(cell_ptr);
        }
    }

    #[test]
    fn ffi_cell_can_get_and_set_chars() {
        unsafe {
            let cell_ptr: *mut VTermScreenCell = vterm_cell_new();

            let a = [b'a' as u32, b'b' as u32, b'c' as u32, 0 as u32, 0 as u32, 0 as u32];
            vterm_cell_set_chars(cell_ptr, a.as_ptr(), 3);
            let mut b = [0 as u32; VTERM_MAX_CHARS_PER_CELL];
            vterm_cell_get_chars(cell_ptr, b.as_mut_ptr(), VTERM_MAX_CHARS_PER_CELL as size_t);
            assert_eq!(a, b);

            vterm_cell_free(cell_ptr);
        }
    }

    #[test]
    fn ffi_cell_can_get_and_set_width() {
        unsafe {
            let cell_ptr: *mut VTermScreenCell = vterm_cell_new();

            vterm_cell_set_width(cell_ptr, 2);
            assert_eq!(2, vterm_cell_get_width(cell_ptr));

            vterm_cell_set_width(cell_ptr, 1);
            assert_eq!(1, vterm_cell_get_width(cell_ptr));

            vterm_cell_free(cell_ptr);
        }
    }

    #[test]
    fn ffi_cell_can_get_and_set_bold() {
        unsafe {
            let cell_ptr: *mut VTermScreenCell = vterm_cell_new();

            vterm_cell_set_bold(cell_ptr, 1);
            assert_eq!(1, vterm_cell_get_bold(cell_ptr));

            vterm_cell_set_bold(cell_ptr, 0);
            assert_eq!(0, vterm_cell_get_bold(cell_ptr));

            vterm_cell_free(cell_ptr);
        }
    }

    #[test]
    fn ffi_cell_can_get_and_set_underline() {
        unsafe {
            let cell_ptr: *mut VTermScreenCell = vterm_cell_new();

            vterm_cell_set_underline(cell_ptr, 1);
            assert_eq!(1, vterm_cell_get_underline(cell_ptr));
            vterm_cell_set_underline(cell_ptr, 2);
            assert_eq!(2, vterm_cell_get_underline(cell_ptr));
            vterm_cell_set_underline(cell_ptr, 3);
            assert_eq!(3, vterm_cell_get_underline(cell_ptr));
            vterm_cell_set_underline(cell_ptr, 0);
            assert_eq!(0, vterm_cell_get_underline(cell_ptr));

            vterm_cell_free(cell_ptr);
        }
    }

    #[test]
    fn ffi_cell_can_get_and_set_italic() {
        unsafe {
            let cell_ptr: *mut VTermScreenCell = vterm_cell_new();

            vterm_cell_set_italic(cell_ptr, 1);
            assert_eq!(1, vterm_cell_get_italic(cell_ptr));
            vterm_cell_set_italic(cell_ptr, 0);
            assert_eq!(0, vterm_cell_get_italic(cell_ptr));

            vterm_cell_free(cell_ptr);
        }
    }

    #[test]
    fn ffi_cell_can_get_and_set_blink() {
        unsafe {
            let cell_ptr: *mut VTermScreenCell = vterm_cell_new();

            vterm_cell_set_blink(cell_ptr, 1);
            assert_eq!(1, vterm_cell_get_blink(cell_ptr));
            vterm_cell_set_blink(cell_ptr, 0);
            assert_eq!(0, vterm_cell_get_blink(cell_ptr));

            vterm_cell_free(cell_ptr);
        }
    }

    #[test]
    fn ffi_cell_can_get_and_set_reverse() {
        unsafe {
            let cell_ptr: *mut VTermScreenCell = vterm_cell_new();

            vterm_cell_set_reverse(cell_ptr, 1);
            assert_eq!(1, vterm_cell_get_reverse(cell_ptr));
            vterm_cell_set_reverse(cell_ptr, 0);
            assert_eq!(0, vterm_cell_get_reverse(cell_ptr));

            vterm_cell_free(cell_ptr);
        }
    }

    #[test]
    fn ffi_cell_can_get_and_set_strike() {
        unsafe {
            let cell_ptr: *mut VTermScreenCell = vterm_cell_new();

            vterm_cell_set_strike(cell_ptr, 1);
            assert_eq!(1, vterm_cell_get_strike(cell_ptr));
            vterm_cell_set_strike(cell_ptr, 0);
            assert_eq!(0, vterm_cell_get_strike(cell_ptr));

            vterm_cell_free(cell_ptr);
        }
    }

    #[test]
    fn ffi_cell_can_get_and_set_font() {
        unsafe {
            let cell_ptr: *mut VTermScreenCell = vterm_cell_new();

            vterm_cell_set_font(cell_ptr, 1);
            assert_eq!(1, vterm_cell_get_font(cell_ptr));
            vterm_cell_set_font(cell_ptr, 5);
            assert_eq!(5, vterm_cell_get_font(cell_ptr));
            vterm_cell_set_font(cell_ptr, 9);
            assert_eq!(9, vterm_cell_get_font(cell_ptr));
            vterm_cell_set_font(cell_ptr, 0);
            assert_eq!(0, vterm_cell_get_font(cell_ptr));

            vterm_cell_free(cell_ptr);
        }
    }

    #[test]
    fn ffi_cell_can_get_and_set_dwl() {
        unsafe {
            let cell_ptr: *mut VTermScreenCell = vterm_cell_new();

            vterm_cell_set_dwl(cell_ptr, 1);
            assert_eq!(1, vterm_cell_get_dwl(cell_ptr));
            vterm_cell_set_dwl(cell_ptr, 0);
            assert_eq!(0, vterm_cell_get_dwl(cell_ptr));

            vterm_cell_free(cell_ptr);
        }
    }

    #[test]
    fn ffi_cell_can_get_and_set_dhl() {
        unsafe {
            let cell_ptr: *mut VTermScreenCell = vterm_cell_new();

            vterm_cell_set_dhl(cell_ptr, 1);
            assert_eq!(1, vterm_cell_get_dhl(cell_ptr));
            vterm_cell_set_dhl(cell_ptr, 2);
            assert_eq!(2, vterm_cell_get_dhl(cell_ptr));
            vterm_cell_set_dhl(cell_ptr, 3);
            assert_eq!(3, vterm_cell_get_dhl(cell_ptr));
            vterm_cell_set_dhl(cell_ptr, 0);
            assert_eq!(0, vterm_cell_get_dhl(cell_ptr));

            vterm_cell_free(cell_ptr);
        }
    }

    #[test]
    fn ffi_cell_bit_fields_dont_interact_with_each_other() {
        unsafe {
            let cell_ptr: *mut VTermScreenCell = vterm_cell_new();

            vterm_cell_set_bold(cell_ptr, 1);
            vterm_cell_set_underline(cell_ptr, 0);
            vterm_cell_set_italic(cell_ptr, 1);
            vterm_cell_set_blink(cell_ptr, 0);
            vterm_cell_set_reverse(cell_ptr, 1);
            vterm_cell_set_strike(cell_ptr, 1);
            vterm_cell_set_font(cell_ptr, 7);
            vterm_cell_set_dwl(cell_ptr, 1);
            vterm_cell_set_dhl(cell_ptr, 2);
            assert_eq!(1, vterm_cell_get_bold(cell_ptr));
            assert_eq!(0, vterm_cell_get_underline(cell_ptr));
            assert_eq!(1, vterm_cell_get_italic(cell_ptr));
            assert_eq!(0, vterm_cell_get_blink(cell_ptr));
            assert_eq!(1, vterm_cell_get_reverse(cell_ptr));
            assert_eq!(1, vterm_cell_get_strike(cell_ptr));
            assert_eq!(7, vterm_cell_get_font(cell_ptr));
            assert_eq!(1, vterm_cell_get_dwl(cell_ptr));
            assert_eq!(2, vterm_cell_get_dhl(cell_ptr));

            vterm_cell_set_blink(cell_ptr, 1);
            vterm_cell_set_strike(cell_ptr, 0);
            vterm_cell_set_dwl(cell_ptr, 0);
            vterm_cell_set_dhl(cell_ptr, 1);
            assert_eq!(1, vterm_cell_get_bold(cell_ptr));
            assert_eq!(0, vterm_cell_get_underline(cell_ptr));
            assert_eq!(1, vterm_cell_get_italic(cell_ptr));
            assert_eq!(1, vterm_cell_get_blink(cell_ptr));
            assert_eq!(1, vterm_cell_get_reverse(cell_ptr));
            assert_eq!(0, vterm_cell_get_strike(cell_ptr));
            assert_eq!(7, vterm_cell_get_font(cell_ptr));
            assert_eq!(0, vterm_cell_get_dwl(cell_ptr));
            assert_eq!(1, vterm_cell_get_dhl(cell_ptr));

            vterm_cell_free(cell_ptr);
        }
    }

    #[test]
    fn ffi_cell_can_get_and_set_fg() {
        unsafe {
            let cell_ptr: *mut VTermScreenCell = vterm_cell_new();

            let color = VTermColor {
                red: 0,
                green: 255,
                blue: 0,
            };
            vterm_cell_set_fg(cell_ptr, color.clone());
            assert_eq!(color, vterm_cell_get_fg(cell_ptr));
            let color = VTermColor {
                red: 255,
                green: 255,
                blue: 0,
            };
            vterm_cell_set_fg(cell_ptr, color.clone());
            assert_eq!(color, vterm_cell_get_fg(cell_ptr));

            vterm_cell_free(cell_ptr);
        }
    }

    #[test]
    fn ffi_cell_can_get_and_set_bg() {
        unsafe {
            let cell_ptr: *mut VTermScreenCell = vterm_cell_new();

            let color = VTermColor {
                red: 0,
                green: 255,
                blue: 0,
            };
            vterm_cell_set_bg(cell_ptr, color.clone());
            assert_eq!(color, vterm_cell_get_bg(cell_ptr));
            let color = VTermColor {
                red: 255,
                green: 255,
                blue: 0,
            };
            vterm_cell_set_bg(cell_ptr, color.clone());
            assert_eq!(color, vterm_cell_get_bg(cell_ptr));

            vterm_cell_free(cell_ptr);
        }
    }
}
