extern crate libc;

pub enum VTermState {}

use super::*;

use libc::{c_int};

extern {
    pub fn vterm_state_get_default_colors(state: *const VTermState, default_fg: *mut VTermColor, default_bg: *mut VTermColor);
    pub fn vterm_state_get_palette_color(state: *const VTermState, index: c_int, color: *mut VTermColor);
}

mod tests {
    extern crate libc;

    use super::super::*;

    #[test]
    fn ffi_state_can_get_default_colors() {
        unsafe {
            let vterm_ptr: *mut VTerm = vterm_new(2, 2);
            let state_ptr = vterm_obtain_state(vterm_ptr);

            let mut fg: VTermColor = Default::default();
            let mut bg: VTermColor = Default::default();
            vterm_state_get_default_colors(state_ptr, &mut fg, &mut bg);

            assert!(fg.red > 200 && fg.red < 255);
            assert!(fg.green > 200 && fg.green < 255);
            assert!(fg.blue > 200 && fg.blue < 255);

            vterm_free(vterm_ptr);
        }
    }
}
