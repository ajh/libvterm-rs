extern crate libc;

use libc::{c_int, c_void};

use super::*;

pub enum VTermScreen {}


// Temporarily stub these in
pub enum VTermRect {}
pub enum VTermProp {}
pub enum VTermValue {}

#[repr(C)]
#[derive(PartialEq, Debug)]
pub struct VTermPos {
    pub row: c_int,
    pub col: c_int,
}

#[repr(C)]
pub struct VTermScreenCallbacks<'a> {
    damage:         &'a Fn(VTermRect, *mut c_void) -> (c_int),
    move_rect:      &'a Fn(VTermRect, VTermRect, *mut c_void) -> (c_int),
    move_cursor:    &'a Fn(VTermPos, VTermPos, c_int, *mut c_void) -> (c_int),
    set_term_prop:  &'a Fn(VTermProp, VTermValue, *mut c_void) -> (c_int),
    bell:           &'a Fn(*mut c_void) -> (c_int),
    resize:         &'a Fn(c_int, c_int, *mut c_void) -> c_int,
    sb_pushline:    &'a Fn(c_int, *const VTermScreenCell, *mut c_void) -> c_int,
    sb_popline:     &'a Fn(c_int, *const VTermScreenCell, *mut c_void) -> c_int,
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

    fn handler_helper(name: String, strings: *mut Vec<String>) {
        unsafe { (*strings).push(name); }
    }
    fn damage_handler(_: VTermRect, strings: *mut c_void) -> c_int { 1 }
    fn move_rect_handler(_: VTermRect, _: VTermRect, strings: *mut c_void) -> c_int { 1 }
    fn move_cursor_handler(_: VTermPos, _: VTermPos, _: c_int, strings: *mut c_void) -> c_int { 1 }
    fn set_term_prop_handler(_: VTermProp, _: VTermValue, strings: *mut c_void) -> c_int { 1 }
    fn bell_handler(strings: *mut c_void) -> c_int { 1 }
    fn resize_handler(_: c_int, _: c_int, strings: *mut c_void) -> c_int { 1 }
    fn sb_pushline_handler(_: c_int, _: *const VTermScreenCell, strings: *mut c_void) -> c_int { 1 }
    fn sb_popline_handler(_: c_int, _: *const VTermScreenCell, strings: *mut c_void) -> c_int { 1 }


    #[test]
    fn screen_can_set_callbacks() {
        unsafe {
            // TODO: write something so the cell will have a known value
            let vterm_ptr: *mut VTerm = vterm_new(5, 5);
            let screen_ptr: *mut VTermScreen = vterm_obtain_screen(vterm_ptr);

            //let callbacks = VTermScreenCallbacks {
                //damage:         &damage_handler,
                //move_rect:      &move_rect_handler,
                //move_cursor:    &move_cursor_handler,
                //set_term_prop:  &set_term_prop_handler,
                //bell:           &bell_handler,
                //resize:         &resize_handler,
                //sb_pushline:    &sb_pushline_handler,
                //sb_popline:     &sb_popline_handler,
            //};

            //let mut strings: Vec<String> = vec!();
            //let strings_ptr: *mut c_void = &mut strings as *mut _ as *mut c_void;
            //vterm_screen_set_callbacks(screen_ptr, &callbacks, strings_ptr);

            //let input = [b'\x07' as libc::c_char]; // bell
            //let input = [b'a' as libc::c_char]; // bell
            let input_bytes = "abc".as_bytes();
            let input_ptr = input_bytes.as_ptr();
            vterm_input_write(vterm_ptr, input_ptr, input_bytes.len() as libc::size_t);

            vterm_free(vterm_ptr);
        }
    }
}
