#![feature(libc)]

extern crate libc;

pub mod ffi;

pub struct ScreenSize {
    rows: usize,
    cols: usize,
}

pub struct VTerm {
    ptr: *mut ffi::VTerm
}

impl VTerm {
    pub fn new(rows: usize, cols: usize) -> VTerm {
        // TODO how to detect error?
        let vterm_ptr = unsafe { ffi::vterm_new(rows as libc::c_int, cols as libc::c_int) };
        VTerm { ptr: vterm_ptr }
    }

    pub fn get_size(&self) -> ScreenSize {
        let mut cols: libc::c_int = 0;
        let mut rows: libc::c_int = 0;
        unsafe { ffi::vterm_get_size(self.ptr, &mut cols, &mut rows); }
        ScreenSize { rows: rows as usize, cols: cols as usize }
    }

    pub fn set_size(&mut self, size: ScreenSize) {
        unsafe { ffi::vterm_set_size(self.ptr, size.cols as libc::c_int, size.rows as libc::c_int); }
    }
}

impl Drop for VTerm {
    fn drop(&mut self) {
        unsafe { ffi::vterm_free(self.ptr) }
    }
}

mod tests {
    use super::*;

    #[test]
    fn can_create_and_destroy_vterm() {
        let vterm: VTerm = VTerm::new(2, 2);
        drop(vterm);
    }

    #[test]
    fn can_get_size() {
        let vterm: VTerm = VTerm::new(2, 2);
        let size = vterm.get_size();
        assert_eq!((2, 2), (size.rows, size.cols));
    }

    #[test]
    fn can_set_size() {
        let mut vterm: VTerm = VTerm::new(2, 2);
        vterm.set_size(ScreenSize { cols: 1, rows: 1 });
        let size = vterm.get_size();
        assert_eq!((1, 1), (size.rows, size.cols));
    }
}
