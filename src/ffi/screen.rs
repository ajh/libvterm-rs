use libc::{c_int, c_void, size_t, uint32_t, c_char};

use super::*;

pub enum VTermScreen {}

#[repr(C)]
pub enum VTermProp {
    VTermPropCursorvisible = 1, // bool
    VTermPropCursorblink, // bool
    VTermPropAltscreen, // bool
    VTermPropTitle, // string
    VTermPropIconname, // string
    VTermPropReverse, // bool
    VTermPropCursorshape, // number
    VTermPropMouse, // number
}
pub enum VTermValue {}

#[repr(C)]
#[derive(PartialEq, Debug)]
pub struct VTermPos {
    pub row: c_int,
    pub col: c_int,
}

impl VTermPos {
    pub fn from_pos(pos: &::Pos) -> VTermPos {
        VTermPos {
            col: pos.x as i32,
            row: pos.y as i32,
        }
    }

    pub fn as_pos(&self) -> ::Pos {
        ::Pos {
            x: self.col as usize,
            y: self.row as usize,
        }
    }
}

#[repr(C)]
#[derive(PartialEq, Debug)]
pub struct VTermRect {
    // End values are exclusive, meaning the maximum index is the end_size - 1.
    pub start_row: c_int,
    pub end_row: c_int,
    pub start_col: c_int,
    pub end_col: c_int,
}

impl VTermRect {
    pub fn from_rect(rect: &::Rect) -> VTermRect {
        VTermRect {
            start_col: rect.origin.x as i32,
            start_row: rect.origin.y as i32,
            end_col: (rect.origin.x + rect.size.width) as i32,
            end_row: (rect.origin.y + rect.size.height) as i32,
        }
    }

    pub fn as_rect(&self) -> ::Rect {
        ::Rect {
            origin: ::Pos {
                x: self.start_col as usize,
                y: self.end_col as usize,
            },
            size: ::Size {
                width: (self.end_col - self.start_col) as usize,
                height: (self.end_row - self.start_row) as usize,
            },
        }
    }
}

#[repr(C)]
pub enum VTermDamageSize {
    VTermDamageCell, // every cell
    VTermDamageRow, // entire rows
    VTermDamageScreen, // entire screen
    VTermDamageScroll, // entire screen + scrollrect
}

pub enum VTermAttrMask {}

#[repr(C)]
pub struct VTermScreenCallbacks {
    pub damage: extern "C" fn(VTermRect, *mut c_void) -> (c_int),
    pub move_rect: extern "C" fn(VTermRect, VTermRect, *mut c_void) -> (c_int),
    pub move_cursor: extern "C" fn(VTermPos, VTermPos, c_int, *mut c_void) -> (c_int),
    pub set_term_prop: extern "C" fn(VTermProp, VTermValue, *mut c_void) -> (c_int),
    pub bell: extern "C" fn(*mut c_void) -> (c_int),
    pub resize: extern "C" fn(c_int, c_int, *mut c_void) -> c_int,
    pub sb_pushline: extern "C" fn(c_int, *const VTermScreenCell, *mut c_void) -> c_int,
    pub sb_popline: extern "C" fn(c_int, *const VTermScreenCell, *mut c_void) -> c_int,
}

extern "C" {
    pub fn vterm_obtain_screen(vt: *mut VTerm) -> *mut VTermScreen;

    pub fn vterm_screen_set_callbacks(screen: *mut VTermScreen,
                                      callbacks: *const VTermScreenCallbacks,
                                      user: *mut c_void);
    pub fn vterm_screen_get_cbdata(screen: *mut VTermScreen) -> *mut c_void;

    pub fn vterm_screen_set_unrecognised_fallbacks(screen: *mut VTermScreen,
                                                   fallbacks: *const VTermParserCallbacks,
                                                   user: *mut c_void)
                                                   -> *mut c_void;
    pub fn vterm_screen_get_unrecognised_fbdata(screen: *mut VTermScreen) -> *mut c_void;

    pub fn vterm_screen_enable_altscreen(screen: *mut VTermScreen, altscreen: c_int);

    pub fn vterm_screen_flush_damage(screen: *mut VTermScreen);
    pub fn vterm_screen_set_damage_merge(screen: *mut VTermScreen, size: VTermDamageSize);

    pub fn vterm_screen_reset(screen: *mut VTermScreen, hard: c_int);

    pub fn vterm_screen_get_chars(screen: *const VTermScreen,
                                  chars: *mut uint32_t,
                                  len: size_t,
                                  rect: VTermRect)
                                  -> size_t;
    pub fn vterm_screen_get_text(screen: *const VTermScreen,
                                 chars: *mut c_char,
                                 len: size_t,
                                 rect: VTermRect)
                                 -> size_t;

    pub fn vterm_screen_get_attrs_extent(screen: *const VTermScreen,
                                         extent: *mut VTermRect,
                                         pos: VTermPos,
                                         attrs: VTermAttrMask)
                                         -> c_int;

    pub fn vterm_screen_get_cell(screen: *const VTermScreen,
                                 pos: VTermPos,
                                 cell: *mut VTermScreenCell)
                                 -> c_int;

    pub fn vterm_screen_is_eol(screen: *const VTermScreen, pos: VTermPos) -> c_int;
}

mod tests {
    #![allow(unused_imports)]
    #![allow(dead_code)]

    use super::super::*;
    use libc::{c_int, c_void, size_t};

    #[test]
    fn ffi_screen_can_reset() {
        unsafe {
            let vterm_ptr: *mut VTerm = vterm_new(2, 2);
            let screen_ptr = vterm_obtain_screen(vterm_ptr);
            vterm_screen_reset(screen_ptr, 1);
            vterm_free(vterm_ptr);
        }
    }

    #[test]
    fn ffi_screen_can_get_cell() {
        unsafe {
            // TODO: write something so the cell will have a known value
            let vterm_ptr: *mut VTerm = vterm_new(2, 2);
            let screen_ptr = vterm_obtain_screen(vterm_ptr);
            let pos = VTermPos { row: 0, col: 0 };
            let cell_ptr: *mut VTermScreenCell = vterm_cell_new();
            let ret = vterm_screen_get_cell(screen_ptr, pos, cell_ptr);
            assert_eq!(1, ret); // one seems to mean success here

            vterm_cell_free(cell_ptr);
            vterm_free(vterm_ptr);
        }
    }

    fn handler_helper(name: String, strings: *mut c_void) {
        println!("handler helper {}", name);
        let mut strings: &mut Vec<String> = unsafe { &mut *(strings as *mut Vec<String>) };
        strings.push(name);
    }

    extern "C" fn damage_handler(_: VTermRect, strings: *mut c_void) -> c_int {
        handler_helper("damage".to_string(), strings);
        1
    }
    extern "C" fn move_rect_handler(_: VTermRect, _: VTermRect, strings: *mut c_void) -> c_int {
        handler_helper("move_rect".to_string(), strings);
        1
    }
    extern "C" fn move_cursor_handler(_: VTermPos,
                                      _: VTermPos,
                                      _: c_int,
                                      strings: *mut c_void)
                                      -> c_int {
        handler_helper("move_cursor".to_string(), strings);
        1
    }
    extern "C" fn set_term_prop_handler(_: VTermProp,
                                        _: VTermValue,
                                        strings: *mut c_void)
                                        -> c_int {
        handler_helper("set_term_prop".to_string(), strings);
        1
    }
    extern "C" fn bell_handler(strings: *mut c_void) -> c_int {
        handler_helper("bell".to_string(), strings);
        1
    }
    extern "C" fn resize_handler(_: c_int, _: c_int, strings: *mut c_void) -> c_int {
        handler_helper("resize".to_string(), strings);
        1
    }
    extern "C" fn sb_pushline_handler(_: c_int,
                                      _: *const VTermScreenCell,
                                      strings: *mut c_void)
                                      -> c_int {
        handler_helper("sb_pushline".to_string(), strings);
        1
    }
    extern "C" fn sb_popline_handler(_: c_int,
                                     _: *const VTermScreenCell,
                                     strings: *mut c_void)
                                     -> c_int {
        handler_helper("sb_popline".to_string(), strings);
        1
    }


    #[test]
    fn ffi_screen_can_set_callbacks() {
        unsafe {
            let vterm_ptr: *mut VTerm = vterm_new(5, 5);
            vterm_set_utf8(vterm_ptr, -1);
            let screen_ptr: *mut VTermScreen = vterm_obtain_screen(vterm_ptr);
            vterm_screen_reset(screen_ptr, 1);

            let callbacks = VTermScreenCallbacks {
                damage: damage_handler,
                move_rect: move_rect_handler,
                move_cursor: move_cursor_handler,
                set_term_prop: set_term_prop_handler,
                bell: bell_handler,
                resize: resize_handler,
                sb_pushline: sb_pushline_handler,
                sb_popline: sb_popline_handler,
            };

            let mut strings: Vec<String> = vec![];
            let strings_ptr: *mut c_void = &mut strings as *mut _ as *mut c_void;
            vterm_screen_set_callbacks(screen_ptr, &callbacks, strings_ptr);

            let input_bytes = "hi".as_bytes();
            let input_ptr = input_bytes.as_ptr();
            vterm_input_write(vterm_ptr, input_ptr, input_bytes.len() as size_t);

            assert_eq!("damage.damage.move_cursor", strings.join("."));

            vterm_free(vterm_ptr);
        }
    }
}
