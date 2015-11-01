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
    pub fn vterm_input_write(vt: *mut VTerm, bytes: *const libc::c_uchar, len: libc::size_t) -> libc::size_t;
    pub fn vterm_obtain_state(vt: *mut VTerm) -> *mut VTermState;
}

mod tests {
    extern crate libc;

    use libc::{c_int};
    use super::super::*;

    #[test]
    fn ffi_vterm_can_create_and_destroy() {
        unsafe {
            let vterm_ptr: *mut VTerm = vterm_new(2, 2);
            vterm_free(vterm_ptr);
        }
    }

    #[test]
    fn ffi_vterm_can_get_size() {
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
    fn ffi_vterm_can_set_size() {
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
    fn ffi_vterm_can_get_and_set_utf8() {
        unsafe {
            let vterm_ptr: *mut VTerm = vterm_new(2, 2);

            vterm_set_utf8(vterm_ptr, 0);
            assert_eq!(0, vterm_get_utf8(vterm_ptr));

            // vterm represents this as "int utf8: 1" which means it is either 0 or -1.
            vterm_set_utf8(vterm_ptr, -1);
            assert_eq!(-1, vterm_get_utf8(vterm_ptr));

            vterm_free(vterm_ptr);
        }
    }

    #[test]
    fn ffi_vterm_can_obtain_screen() {
        unsafe {
            let vterm_ptr: *mut VTerm = vterm_new(2, 2);
            vterm_obtain_screen(vterm_ptr);
            vterm_free(vterm_ptr);
        }
    }

    #[test]
    fn ffi_vterm_can_write_input() {
        unsafe {
            let vterm_ptr: *mut VTerm = vterm_new(2, 2);

            // there probably a nicer way to do this
            let input = [
                b'a' as libc::c_uchar,
                b'b' as libc::c_uchar,
                b'c' as libc::c_uchar,
            ];
            let bytes_read = vterm_input_write(vterm_ptr, input.as_ptr(), 3);
            assert_eq!(3, bytes_read);
            vterm_free(vterm_ptr);
        }
    }

    #[test]
    fn ffi_vterm_can_obtain_state() {
        unsafe {
            let vterm_ptr: *mut VTerm = vterm_new(2, 2);
            vterm_obtain_state(vterm_ptr);
            vterm_free(vterm_ptr);
        }
    }
}
