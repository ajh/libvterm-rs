extern crate libc;

use libc::{c_int, c_void};
use std::sync::mpsc;

use super::*;

pub struct VTerm {
    ptr: *mut ffi::VTerm,
    screen_event_tx: Option<mpsc::Sender<ScreenEvent>>,
}

impl VTerm {
    pub fn new(rows: u16, cols: u16) -> VTerm {
        // TODO how to detect error?
        let vterm_ptr = unsafe { ffi::vterm_new(rows as c_int, cols as c_int) };
        VTerm { ptr: vterm_ptr, screen_event_tx: None }
    }

    pub fn get_size(&self) -> ScreenSize {
        let mut cols: c_int = 0;
        let mut rows: c_int = 0;
        unsafe { ffi::vterm_get_size(self.ptr, &mut cols, &mut rows); }
        ScreenSize { rows: rows as u16, cols: cols as u16 }
    }

    pub fn set_size(&mut self, size: ScreenSize) {
        unsafe { ffi::vterm_set_size(self.ptr, size.cols as c_int, size.rows as c_int); }
    }

    pub fn get_utf8(&self) -> bool {
        unsafe { super::int_to_bool(ffi::vterm_get_utf8(self.ptr)) }
    }

    pub fn set_utf8(&mut self, is_utf8: bool) {
        unsafe { ffi::vterm_set_utf8(self.ptr, super::bool_to_int(is_utf8)) }
    }

    // TODO: figure out lifetime and data race issues
    //:
    // I think I don't want this, instead provide access to an owned screen stored in the struct.
    //
    // The field could either be public, or have a method to borrow the screen. I've seen borrow()
    // and borrow_mut() around, so maybe that's idomatic.
    pub fn get_screen(&self) -> Screen {
        let screen_ptr = unsafe { ffi::vterm_obtain_screen(self.ptr) };
        Screen::from_ptr(screen_ptr)
    }

    // TODO: figure out lifetime and data race issues
    pub fn get_state(&self) -> State {
        let state_ptr = unsafe { ffi::vterm_obtain_state(self.ptr) };
        State::from_ptr(state_ptr)
    }

    pub fn write(&mut self, input: &[u8]) -> u32 {
        unsafe {
            ffi::vterm_input_write(self.ptr, input.as_ptr(), input.len() as libc::size_t) as u32
        }
    }

    /// install vtermscreen callbacks that will add events to the returned channel.
    pub fn receive_screen_events(&mut self) -> mpsc::Receiver<ScreenEvent> {
        let (tx, rx) = mpsc::channel();
        self.screen_event_tx = Some(tx);

        unsafe {
            let screen_ptr = ffi::vterm_obtain_screen(self.ptr);
            let tx_ptr: *mut c_void = &mut self.screen_event_tx as *mut _ as *mut c_void;
            ffi::vterm_screen_set_callbacks(screen_ptr, &SCREEN_CALLBACKS, tx_ptr);
        }

        rx
    }
}

impl Drop for VTerm {
    fn drop(&mut self) {
        unsafe { ffi::vterm_free(self.ptr) }
    }
}

mod tests {
    use super::super::*;

    #[test]
    fn vterm_can_create_and_destroy() {
        let vterm: VTerm = VTerm::new(2, 2);
        drop(vterm);
    }

    #[test]
    fn vterm_can_get_size() {
        let vterm: VTerm = VTerm::new(2, 2);
        let size = vterm.get_size();
        assert_eq!((2, 2), (size.rows, size.cols));
    }

    #[test]
    fn vterm_can_set_size() {
        let mut vterm: VTerm = VTerm::new(2, 2);
        vterm.set_size(ScreenSize { cols: 1, rows: 1 });
        let size = vterm.get_size();
        assert_eq!((1, 1), (size.rows, size.cols));
    }

    #[test]
    fn vterm_can_get_and_set_utf8() {
        let mut vterm: VTerm = VTerm::new(2, 2);
        vterm.set_utf8(true);
        assert_eq!(true, vterm.get_utf8());

        vterm.set_utf8(false);
        assert_eq!(false, vterm.get_utf8());
    }

    #[test]
    fn vterm_can_get_screen() {
        let vterm: VTerm = VTerm::new(2, 2);
        vterm.get_screen();
    }

    #[test]
    fn vterm_can_get_state() {
        let vterm: VTerm = VTerm::new(2, 2);
        vterm.get_state();
    }

    #[test]
    fn vterm_can_write() {
        let mut vterm: VTerm = VTerm::new(2, 2);
        let input: &[u8] = "abcd".as_bytes();
        assert_eq!(4, vterm.write(input));
    }
}
