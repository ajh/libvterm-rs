extern crate libc;

use libc::{c_int};

use super::*;

pub struct VTermColor {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

pub struct VTermScreenCell {
    ptr: *mut ffi::VTermScreenCell
}

impl VTermScreenCell {
    pub fn new() -> VTermScreenCell {
        let ptr = unsafe { ffi::vterm_cell_new() };
        VTermScreenCell { ptr: ptr }
    }
}

impl Drop for VTermScreenCell {
    fn drop(&mut self) {
        unsafe { ffi::vterm_cell_free(self.ptr) }
    }
}

mod tests {
    use super::*;

    #[test]
    fn cell_can_create_and_destroy() {
        let cell = VTermScreenCell::new();
        drop(cell);
    }
}
