extern crate libc;

use libc::{c_int};

use super::*;

pub struct Color {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

pub struct Cell {
    ptr: *mut ffi::VTermScreenCell
}

impl Cell {
    pub fn new() -> Cell {
        let ptr = unsafe { ffi::vterm_cell_new() };
        Cell { ptr: ptr }
    }
}

impl Drop for Cell {
    fn drop(&mut self) {
        unsafe { ffi::vterm_cell_free(self.ptr) }
    }
}

mod tests {
    use super::*;

    #[test]
    fn cell_can_create_and_destroy() {
        let cell = Cell::new();
        drop(cell);
    }
}
