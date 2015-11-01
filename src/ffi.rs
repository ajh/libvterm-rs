extern crate libc;

use libc::{c_int};

pub enum VTerm {}
pub enum VTermScreen {}
pub enum VTermScreenCell {}
pub enum VTermState {}

pub const VTERM_MAX_CHARS_PER_CELL: usize = 6;

#[repr(C)]
#[derive(PartialEq, Debug)]
pub struct VTermPos {
    pub row: c_int,
    pub col: c_int,
}

#[repr(C)]
#[derive(PartialEq, Debug, Clone, Default)]
pub struct VTermColor {
    pub red:   libc::uint8_t,
    pub green: libc::uint8_t,
    pub blue:  libc::uint8_t,
}

extern {
    pub fn vterm_new(rows: c_int, cols: c_int) -> *mut VTerm;
    pub fn vterm_free(vt: *mut VTerm);
    pub fn vterm_get_size(vt: *const VTerm, rowsp: *mut c_int, colsp: *mut c_int);
    pub fn vterm_set_size(vt: *mut VTerm, rows: c_int, cols: c_int);
    pub fn vterm_get_utf8(vt: *const VTerm) -> c_int;
    pub fn vterm_set_utf8(vt: *mut VTerm, is_utf8: c_int);
    pub fn vterm_obtain_screen(vt: *mut VTerm) -> *mut VTermScreen;
    pub fn vterm_input_write(vt: *mut VTerm, bytes: *const libc::c_char, len: libc::size_t) -> libc::size_t;
    pub fn vterm_obtain_state(vt: *mut VTerm) -> *mut VTermState;

    pub fn vterm_screen_reset(screen: *mut VTermScreen, hard: c_int);
    pub fn vterm_screen_get_cell(screen: *const VTermScreen, pos: VTermPos, cell: *mut VTermScreenCell) -> c_int;

    // These are my rust ffi bitfield workarounds
    pub fn vterm_cell_new() -> *mut VTermScreenCell;
    pub fn vterm_cell_free(cell: *mut VTermScreenCell);
    pub fn vterm_cell_get_chars(cell: *const VTermScreenCell, chars: *mut libc::uint32_t, len: libc::size_t) -> c_int;
    pub fn vterm_cell_set_chars(cell: *mut VTermScreenCell, chars: *const libc::uint32_t, len: libc::size_t);
    pub fn vterm_cell_get_width(cell: *const VTermScreenCell) -> libc::c_char;
    pub fn vterm_cell_set_width(cell: *mut VTermScreenCell, width: libc::c_char);
    pub fn vterm_cell_get_bold(cell: *const VTermScreenCell) -> libc::c_uint;
    pub fn vterm_cell_set_bold(cell: *mut VTermScreenCell, is_bold: libc::c_uint);
    pub fn vterm_cell_get_underline(cell: *const VTermScreenCell) -> libc::c_uint;
    pub fn vterm_cell_set_underline(cell: *mut VTermScreenCell, underline_value: libc::c_uint);
    pub fn vterm_cell_get_italic(cell: *const VTermScreenCell) -> libc::c_uint;
    pub fn vterm_cell_set_italic(cell: *mut VTermScreenCell, is_italic: libc::c_uint);
    pub fn vterm_cell_get_blink(cell: *const VTermScreenCell) -> libc::c_uint;
    pub fn vterm_cell_set_blink(cell: *mut VTermScreenCell, is_blink: libc::c_uint);
    pub fn vterm_cell_get_reverse(cell: *const VTermScreenCell) -> libc::c_uint;
    pub fn vterm_cell_set_reverse(cell: *mut VTermScreenCell, is_reverse: libc::c_uint);
    pub fn vterm_cell_get_strike(cell: *const VTermScreenCell) -> libc::c_uint;
    pub fn vterm_cell_set_strike(cell: *mut VTermScreenCell, is_strike: libc::c_uint);
    pub fn vterm_cell_get_font(cell: *const VTermScreenCell) -> libc::c_uint;
    pub fn vterm_cell_set_font(cell: *mut VTermScreenCell, font_value: libc::c_uint);
    pub fn vterm_cell_get_dwl(cell: *const VTermScreenCell) -> libc::c_uint;
    pub fn vterm_cell_set_dwl(cell: *mut VTermScreenCell, dwl: libc::c_uint);
    pub fn vterm_cell_get_dhl(cell: *const VTermScreenCell) -> libc::c_uint;
    pub fn vterm_cell_set_dhl(cell: *mut VTermScreenCell, dhl: libc::c_uint);
    pub fn vterm_cell_get_fg(cell: *const VTermScreenCell) -> VTermColor;
    pub fn vterm_cell_set_fg(cell: *mut VTermScreenCell, color: VTermColor);
    pub fn vterm_cell_get_bg(cell: *const VTermScreenCell) -> VTermColor;
    pub fn vterm_cell_set_bg(cell: *mut VTermScreenCell, color: VTermColor);

    pub fn vterm_state_get_default_colors(state: *const VTermState, default_fg: *mut VTermColor, default_bg: *mut VTermColor);
}

mod tests {
    extern crate libc;

    use libc::{c_int};
    use super::*;

    #[test]
    fn vterm_can_create_and_destroy() {
        unsafe {
            let vterm_ptr: *mut VTerm = vterm_new(2, 2);
            vterm_free(vterm_ptr);
        }
    }

    #[test]
    fn vterm_can_get_size() {
        unsafe {
            let vterm_ptr: *mut VTerm = vterm_new(2, 2);
            let mut cols: c_int = 0;
            let mut rows: c_int = 0;
            vterm_get_size(vterm_ptr, &mut cols, &mut rows);
            assert_eq!((2, 2), (cols, rows));

            vterm_free(vterm_ptr);
        }
    }

    #[test]
    fn vterm_can_set_size() {
        unsafe {
            let vterm_ptr: *mut VTerm = vterm_new(2, 2);
            vterm_set_size(vterm_ptr, 1, 1);

            let mut cols: c_int = 0;
            let mut rows: c_int = 0;
            vterm_get_size(vterm_ptr, &mut cols, &mut rows);
            assert_eq!((1, 1), (cols, rows));

            vterm_free(vterm_ptr);
        }
    }

    #[test]
    fn vterm_can_get_and_set_utf8() {
        unsafe {
            let vterm_ptr: *mut VTerm = vterm_new(2, 2);

            vterm_set_utf8(vterm_ptr, 1);
            let val = vterm_get_utf8(vterm_ptr);
            assert_eq!(1, val); // not sure why this doesnt work

            vterm_free(vterm_ptr);
        }
    }

    #[test]
    fn vterm_can_obtain_screen() {
        unsafe {
            let vterm_ptr: *mut VTerm = vterm_new(2, 2);
            vterm_obtain_screen(vterm_ptr);
            vterm_free(vterm_ptr);
        }
    }

    #[test]
    fn vterm_can_write_input() {
        unsafe {
            let vterm_ptr: *mut VTerm = vterm_new(2, 2);

            // there probably a nicer way to do this
            let input = [
                b'a' as libc::c_char,
                b'b' as libc::c_char,
                b'c' as libc::c_char,
            ];
            let bytes_read = vterm_input_write(vterm_ptr, input.as_ptr(), 3);
            assert_eq!(3, bytes_read);
            vterm_free(vterm_ptr);
        }
    }

    #[test]
    fn vterm_can_obtain_state() {
        unsafe {
            let vterm_ptr: *mut VTerm = vterm_new(2, 2);
            vterm_obtain_state(vterm_ptr);
            vterm_free(vterm_ptr);
        }
    }

    #[test]
    fn screen_can_reset() {
        unsafe {
            let vterm_ptr: *mut VTerm = vterm_new(2, 2);
            let screen_ptr = vterm_obtain_screen(vterm_ptr);
            vterm_screen_reset(screen_ptr, 1);
            vterm_free(vterm_ptr);
        }
    }

    #[test]
    fn screen_can_get_cell() {
        unsafe {
            // TODO: write something so the cell will have a known value
            let vterm_ptr: *mut VTerm = vterm_new(2, 2);
            let screen_ptr = vterm_obtain_screen(vterm_ptr);
            let pos = VTermPos { row: 1, col: 0 };
            let cell_ptr: *mut VTermScreenCell = vterm_cell_new();
            let ret = vterm_screen_get_cell(screen_ptr, pos, cell_ptr);
            assert_eq!(0, ret);

            vterm_cell_free(cell_ptr);
            vterm_free(vterm_ptr);
        }
    }

    #[test]
    fn cell_can_create_and_destroy() {
        unsafe {
            let cell_ptr: *mut VTermScreenCell = vterm_cell_new();
            vterm_cell_free(cell_ptr);
        }
    }

    #[test]
    fn cell_can_get_and_set_chars() {
        unsafe {
            let cell_ptr: *mut VTermScreenCell = vterm_cell_new();

            let a = [b'a' as u32, b'b' as u32, b'c' as u32, 0 as u32, 0 as u32, 0 as u32];
            vterm_cell_set_chars(cell_ptr, a.as_ptr(), 3);
            let mut b = [0 as u32; VTERM_MAX_CHARS_PER_CELL];
            vterm_cell_get_chars(cell_ptr, b.as_mut_ptr(), VTERM_MAX_CHARS_PER_CELL as libc::size_t);
            assert_eq!(a, b);

            vterm_cell_free(cell_ptr);
        }
    }

    #[test]
    fn cell_can_get_and_set_width() {
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
    fn cell_can_get_and_set_bold() {
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
    fn cell_can_get_and_set_underline() {
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
    fn cell_can_get_and_set_italic() {
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
    fn cell_can_get_and_set_blink() {
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
    fn cell_can_get_and_set_reverse() {
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
    fn cell_can_get_and_set_strike() {
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
    fn cell_can_get_and_set_font() {
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
    fn cell_can_get_and_set_dwl() {
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
    fn cell_can_get_and_set_dhl() {
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
    fn cell_bit_fields_dont_interact_with_each_other() {
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
    fn cell_can_get_and_set_fg() {
        unsafe {
            let cell_ptr: *mut VTermScreenCell = vterm_cell_new();

            let color = VTermColor { red: 0, green: 255, blue: 0 };
            vterm_cell_set_fg(cell_ptr, color.clone());
            assert_eq!(color, vterm_cell_get_fg(cell_ptr));
            let color = VTermColor { red: 255, green: 255, blue: 0 };
            vterm_cell_set_fg(cell_ptr, color.clone());
            assert_eq!(color, vterm_cell_get_fg(cell_ptr));

            vterm_cell_free(cell_ptr);
        }
    }

    #[test]
    fn cell_can_get_and_set_bg() {
        unsafe {
            let cell_ptr: *mut VTermScreenCell = vterm_cell_new();

            let color = VTermColor { red: 0, green: 255, blue: 0 };
            vterm_cell_set_bg(cell_ptr, color.clone());
            assert_eq!(color, vterm_cell_get_bg(cell_ptr));
            let color = VTermColor { red: 255, green: 255, blue: 0 };
            vterm_cell_set_bg(cell_ptr, color.clone());
            assert_eq!(color, vterm_cell_get_bg(cell_ptr));

            vterm_cell_free(cell_ptr);
        }
    }


    #[test]
    fn state_can_get_default_colors() {
        unsafe {
            let vterm_ptr: *mut VTerm = vterm_new(2, 2);
            let state_ptr = vterm_obtain_state(vterm_ptr);

            let mut fg: VTermColor = Default::default();
            let mut bg: VTermColor = Default::default();
            vterm_state_get_default_colors(state_ptr, &mut fg, &mut bg);

            assert!(fg.red > 200 && fg.red < 255);
            assert!(fg.green > 200 && fg.green < 255);
            assert!(fg.blue > 200 && fg.blue < 255);

            vterm_free(vterm_ptr);
        }
    }
}
