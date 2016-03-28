use libc::{c_int, c_void};
use super::*;

pub enum VTermState {}
pub enum VTermLineInfo {}  // need to flesh this out

#[derive(Debug)]
#[repr(C)]
pub enum VTermAttr {
    Bold = 1, // bool:   1, 22
    Underline, // number: 4, 21, 24
    Italic, // bool:   3, 23
    Blink, // bool:   5, 25
    Reverse, // bool:   7, 27
    Strike, // bool:   9, 29
    Font, // number: 10-19
    Foreground, // color:  30-39 90-97
    Background, // color:  40-49 100-107
}

#[repr(C)]
pub struct VTermStateCallbacks {
    pub put_glyph: Option<extern "C" fn(*mut VTermGlyphInfo, VTermPos, *mut c_void) -> (c_int)>,
    pub move_cursor: Option<extern "C" fn(VTermPos, VTermPos, c_int, *mut c_void) -> (c_int)>,
    pub scroll_rect: Option<extern "C" fn(VTermRect, c_int, c_int, *mut c_void) -> (c_int)>,
    pub move_rect: Option<extern "C" fn(VTermRect, VTermRect, *mut c_void) -> (c_int)>,
    pub erase: Option<extern "C" fn(VTermRect, c_int, *mut c_void) -> (c_int)>,
    pub init_pen: Option<extern "C" fn(*mut c_void) -> (c_int)>,
    pub set_pen_attr: Option<extern "C" fn(VTermAttr, *mut VTermValue, *mut c_void) -> (c_int)>,
    pub set_term_prop: Option<extern "C" fn(VTermProp, *mut VTermValue, *mut c_void) -> (c_int)>,
    pub bell: Option<extern "C" fn(*mut c_void) -> (c_int)>,
    pub resize: Option<extern "C" fn(c_int, c_int, *mut VTermPos, *mut c_void) -> (c_int)>,
    pub set_line_info: Option<extern "C" fn(c_int,
                                            *const VTermLineInfo,
                                            *const VTermLineInfo,
                                            *mut c_void)
                                            -> (c_int)>,
}
impl Default for VTermStateCallbacks {
    fn default() -> VTermStateCallbacks {
        VTermStateCallbacks {
            put_glyph: None,
            move_cursor: None,
            scroll_rect: None,
            move_rect: None,
            erase: None,
            init_pen: None,
            set_pen_attr: None,
            set_term_prop: None,
            bell: None,
            resize: None,
            set_line_info: None,
        }
    }
}

extern "C" {
    pub fn vterm_obtain_state(vt: *mut VTerm) -> *mut VTermState;

    pub fn vterm_state_set_callbacks(state: *mut VTermState,
                                     callbacks: *const VTermStateCallbacks,
                                     user: *mut c_void);
    pub fn vterm_state_get_cbdata(state: *mut VTermState) -> *mut c_void;

    pub fn vterm_state_set_unrecognised_fallbacks(state: *mut VTermState,
                                                  fallbacks: *const VTermParserCallbacks,
                                                  user: *mut c_void);
    pub fn vterm_state_get_unrecognised_fbdata(state: *mut VTermState) -> *mut c_void;

    pub fn vterm_state_reset(state: *mut VTermState, hard: c_int);
    pub fn vterm_state_get_cursorpos(state: *const VTermState, cursorpos: *mut VTermPos);
    pub fn vterm_state_get_default_colors(state: *const VTermState,
                                          default_fg: *mut VTermColor,
                                          default_bg: *mut VTermColor);
    pub fn vterm_state_get_palette_color(state: *const VTermState,
                                         index: c_int,
                                         color: *mut VTermColor);
    pub fn vterm_state_set_default_colors(state: *mut VTermState,
                                          default_fg: *const VTermColor,
                                          default_bg: *const VTermColor);
    pub fn vterm_state_set_palette_color(state: *mut VTermState,
                                         index: c_int,
                                         col: *const VTermColor);
    pub fn vterm_state_set_bold_highbright(state: *mut VTermState, bold_is_highbright: c_int);
    pub fn vterm_state_get_penattr(state: *const VTermState,
                                   attr: VTermAttr,
                                   val: *mut VTermValue)
                                   -> c_int;
    pub fn vterm_state_set_termprop(state: *mut VTermState,
                                    prop: VTermProp,
                                    val: *mut VTermValue)
                                    -> c_int;
    pub fn vterm_state_get_lineinfo(state: *const VTermState, row: c_int) -> *const VTermLineInfo;
}

mod tests {
    #![allow(unused_imports)]
    use super::*;

    #[test]
    fn ffi_state_can_get_and_set_default_colors() {
        unsafe {
            let vterm_ptr: *mut ::ffi::VTerm = ::ffi::vterm_new(2, 2);
            let state_ptr = vterm_obtain_state(vterm_ptr);

            let fg = ::ffi::VTermColor {
                red: 200,
                green: 201,
                blue: 202,
            };
            let bg = ::ffi::VTermColor {
                red: 10,
                green: 11,
                blue: 12,
            };
            vterm_state_set_default_colors(state_ptr, &fg, &bg);

            let mut fg: ::ffi::VTermColor = Default::default();
            let mut bg: ::ffi::VTermColor = Default::default();
            vterm_state_get_default_colors(state_ptr, &mut fg, &mut bg);

            assert_eq!(fg.red, 200);
            assert_eq!(fg.green, 201);
            assert_eq!(fg.blue, 202);

            assert_eq!(bg.red, 10);
            assert_eq!(bg.green, 11);
            assert_eq!(bg.blue, 12);

            ::ffi::vterm_free(vterm_ptr);
        }
    }
}
