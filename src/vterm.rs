extern crate libc;

use libc::{c_int, c_void};
use std::sync::mpsc;

use super::*;

pub struct VTerm {
    pub screen: Screen,
    pub state: State,
    ptr: *mut ffi::VTerm,
    screen_event_tx: Option<mpsc::Sender<ScreenEvent>>,
}

impl VTerm {
    pub fn new(size: ScreenSize) -> VTerm {
        // TODO how to detect error?
        let vterm_ptr = unsafe { ffi::vterm_new(size.rows as c_int, size.cols as c_int) };
        let screen_ptr = unsafe { ffi::vterm_obtain_screen(vterm_ptr) };
        let state_ptr = unsafe { ffi::vterm_obtain_state(vterm_ptr) };

        let mut vterm = VTerm {
            ptr: vterm_ptr,
            screen_event_tx: None,
            screen: Screen::from_ptr(screen_ptr),
            state: State::from_ptr(state_ptr),
        };

        vterm.screen.reset(true);

        vterm
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

    pub fn write(&mut self, input: &[u8]) -> u32 {
        unsafe {
            ffi::vterm_input_write(self.ptr, input.as_ptr(), input.len() as libc::size_t) as u32
        }
    }

    /// Return a channel Receiver that will be sent ScreenEvents.
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
        let vterm: VTerm = VTerm::new(ScreenSize { rows: 2, cols: 2 });
        drop(vterm);
    }

    #[test]
    fn vterm_can_get_size() {
        let vterm: VTerm = VTerm::new(ScreenSize { rows: 2, cols: 2 });
        let size = vterm.get_size();
        assert_eq!((2, 2), (size.rows, size.cols));
    }

    #[test]
    fn vterm_can_set_size() {
        let mut vterm: VTerm = VTerm::new(ScreenSize { rows: 2, cols: 2 });
        vterm.set_size(ScreenSize { cols: 1, rows: 1 });
        let size = vterm.get_size();
        assert_eq!((1, 1), (size.rows, size.cols));
    }

    #[test]
    fn vterm_can_get_and_set_utf8() {
        let mut vterm: VTerm = VTerm::new(ScreenSize { rows: 2, cols: 2 });
        vterm.set_utf8(true);
        assert_eq!(true, vterm.get_utf8());

        vterm.set_utf8(false);
        assert_eq!(false, vterm.get_utf8());
    }

    #[test]
    fn vterm_can_write() {
        let mut vterm: VTerm = VTerm::new(ScreenSize { rows: 2, cols: 2 });
        let input: &[u8] = "abcd".as_bytes();
        assert_eq!(4, vterm.write(input));
    }
}
