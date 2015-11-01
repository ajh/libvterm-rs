extern crate libc;

use libc::{c_int, c_void};

use super::*;

pub enum VTermScreen {}


// Temporarily stub these in
pub enum VTermProp {}
pub enum VTermValue {}

#[repr(C)]
#[derive(PartialEq, Debug)]
pub struct VTermPos {
    pub row: c_int,
    pub col: c_int,
}

#[repr(C)]
#[derive(PartialEq, Debug)]
pub struct VTermRect {
    pub start_row: c_int,
    pub end_row: c_int,
    pub start_col: c_int,
    pub end_col: c_int,
}

extern "C" fn default_damage_handler(_: VTermRect, strings: *mut c_void) -> c_int { 1 }
extern "C" fn default_move_rect_handler(_: VTermRect, _: VTermRect, strings: *mut c_void) -> c_int { 1 }
extern "C" fn default_move_cursor_handler(_: VTermPos, _: VTermPos, _: c_int, strings: *mut c_void) -> c_int { 1 }
extern "C" fn default_set_term_prop_handler(_: VTermProp, _: VTermValue, strings: *mut c_void) -> c_int { 1 }
extern "C" fn default_bell_handler(strings: *mut c_void) -> c_int { 1 }
extern "C" fn default_resize_handler(_: c_int, _: c_int, strings: *mut c_void) -> c_int { 1 }
extern "C" fn default_sb_pushline_handler(_: c_int, _: *const VTermScreenCell, strings: *mut c_void) -> c_int { 1 }
extern "C" fn default_sb_popline_handler(_: c_int, _: *const VTermScreenCell, strings: *mut c_void) -> c_int { 1 }

#[repr(C)]
pub struct VTermScreenCallbacks {
    pub damage:         extern fn(VTermRect, *mut c_void) -> (c_int),
    pub move_rect:      extern fn(VTermRect, VTermRect, *mut c_void) -> (c_int),
    pub move_cursor:    extern fn(VTermPos, VTermPos, c_int, *mut c_void) -> (c_int),
    pub set_term_prop:  extern fn(VTermProp, VTermValue, *mut c_void) -> (c_int),
    pub bell:           extern fn(*mut c_void) -> (c_int),
    pub resize:         extern fn(c_int, c_int, *mut c_void) -> c_int,
    pub sb_pushline:    extern fn(c_int, *const VTermScreenCell, *mut c_void) -> c_int,
    pub sb_popline:     extern fn(c_int, *const VTermScreenCell, *mut c_void) -> c_int,
}

impl Default for VTermScreenCallbacks {
    fn default() -> VTermScreenCallbacks {
        VTermScreenCallbacks {
            damage:             default_damage_handler,
            move_rect:          default_move_rect_handler,
            move_cursor:        default_move_cursor_handler,
            set_term_prop:      default_set_term_prop_handler,
            bell:               default_bell_handler,
            resize:             default_resize_handler,
            sb_pushline:        default_sb_pushline_handler,
            sb_popline:         default_sb_popline_handler,
        }
    }
}

extern {
    pub fn vterm_screen_reset(screen: *mut VTermScreen, hard: c_int);
    pub fn vterm_screen_get_cell(screen: *const VTermScreen, pos: VTermPos, cell: *mut VTermScreenCell) -> c_int;
    pub fn vterm_screen_set_callbacks(screen: *mut VTermScreen, callbacks: *const VTermScreenCallbacks, user: *mut c_void);
}

mod tests {
    extern crate libc;

    use super::super::*;
    use libc::{c_int, c_void};

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
            let pos = VTermPos { row: 1, col: 0 };
            let cell_ptr: *mut VTermScreenCell = vterm_cell_new();
            let ret = vterm_screen_get_cell(screen_ptr, pos, cell_ptr);
            assert_eq!(0, ret);

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
    extern "C" fn move_cursor_handler(_: VTermPos, _: VTermPos, _: c_int, strings: *mut c_void) -> c_int {
        handler_helper("move_cursor".to_string(), strings);
        1
    }
    extern "C" fn set_term_prop_handler(_: VTermProp, _: VTermValue, strings: *mut c_void) -> c_int {
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
    extern "C" fn sb_pushline_handler(_: c_int, _: *const VTermScreenCell, strings: *mut c_void) -> c_int {
        handler_helper("sb_pushline".to_string(), strings);
        1
    }
    extern "C" fn sb_popline_handler(_: c_int, _: *const VTermScreenCell, strings: *mut c_void) -> c_int {
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
                damage:         damage_handler,
                move_rect:      move_rect_handler,
                move_cursor:    move_cursor_handler,
                set_term_prop:  set_term_prop_handler,
                bell:           bell_handler,
                resize:         resize_handler,
                sb_pushline:    sb_pushline_handler,
                sb_popline:     sb_popline_handler,
            };

            let mut strings: Vec<String> = vec!();
            let strings_ptr: *mut c_void = &mut strings as *mut _ as *mut c_void;
            vterm_screen_set_callbacks(screen_ptr, &callbacks, strings_ptr);

            let input_bytes = "hi".as_bytes();
            let input_ptr = input_bytes.as_ptr();
            vterm_input_write(vterm_ptr, input_ptr, input_bytes.len() as libc::size_t);

            assert_eq!("damage.damage.move_cursor", strings.connect("."));

            vterm_free(vterm_ptr);
        }
    }
}
