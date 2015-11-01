extern crate libc;

use libc::{c_int};

use super::*;

pub enum VTermScreen {}

#[repr(C)]
#[derive(PartialEq, Debug)]
pub struct VTermPos {
    pub row: c_int,
    pub col: c_int,
}

extern {
    pub fn vterm_screen_reset(screen: *mut VTermScreen, hard: c_int);
    pub fn vterm_screen_get_cell(screen: *const VTermScreen, pos: VTermPos, cell: *mut VTermScreenCell) -> c_int;
}

mod tests {
    extern crate libc;

    use super::super::*;

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
}
