extern crate libc;

pub enum VTerm {}

extern {
    pub fn vterm_new(rows: libc::c_int, cols: libc::c_int) -> *mut VTerm;
    pub fn vterm_free(vterm: *mut VTerm);
    pub fn vterm_get_size(vterm: *const VTerm, rowsp: *mut libc::c_int, colsp: *mut libc::c_int);
    pub fn vterm_set_size(vterm: *mut VTerm, rows: libc::c_int, cols: libc::c_int);
}

mod tests {
    extern crate libc;

    use super::*;

    #[test]
    fn can_create_and_destroy_vterm() {
        let vterm_ptr: *mut VTerm = unsafe { vterm_new(2, 2) };
        unsafe { vterm_free(vterm_ptr) };
    }

    #[test]
    fn can_get_size() {
        let vterm_ptr: *mut VTerm = unsafe { vterm_new(2, 2) };
        let mut cols: libc::c_int = 0;
        let mut rows: libc::c_int = 0;
        unsafe { vterm_get_size(vterm_ptr, &mut cols, &mut rows); }
        assert_eq!((2, 2), (cols, rows));

        unsafe { vterm_free(vterm_ptr) };
    }

    #[test]
    fn can_set_size() {
        let vterm_ptr: *mut VTerm = unsafe { vterm_new(2, 2) };
        unsafe { vterm_set_size(vterm_ptr, 1, 1) };

        let mut cols: libc::c_int = 0;
        let mut rows: libc::c_int = 0;
        unsafe { vterm_get_size(vterm_ptr, &mut cols, &mut rows); }
        assert_eq!((1, 1), (cols, rows));

        unsafe { vterm_free(vterm_ptr) };
    }
}
