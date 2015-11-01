extern crate libc;

use libc::{c_int};

use super::*;

pub enum VTerm {}

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
}

mod tests {
    extern crate libc;

    use libc::{c_int};
    use super::super::*;

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
}
