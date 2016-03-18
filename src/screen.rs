use libc::{c_void, size_t, c_char};
use std::sync::mpsc;

use super::*;

impl VTerm {
    /// Reset the screen. I've observed this needs to happen before using or segfaults will occur.
    pub fn screen_reset(&mut self, is_hard: bool) {
        unsafe { ffi::vterm_screen_reset(self.screen_ptr.get_mut(), super::bool_to_int(is_hard)) }
    }

    /// Return the cell at the given position
    pub fn screen_get_cell(&self, pos: &Pos) -> ScreenCell {
        let cell_buf = unsafe { ffi::vterm_cell_new() };
        unsafe {
            ffi::vterm_screen_get_cell(self.screen_ptr.get(),
                                       ffi::VTermPos::from_pos(&pos),
                                       cell_buf)
        };
        let cell = ScreenCell::from_ptr(cell_buf, &self); // shouldn't this take &cell_buf?
        unsafe { ffi::vterm_cell_free(cell_buf) };

        cell
    }

    // Returns the text within the rect as a String. Invalid utf8 sequences are replaces with or
    // panics if invalid utf8 bytes are found
    pub fn screen_get_text_lossy(&mut self, rect: &Rect) -> String {
        let bytes = self.get_text_as_bytes(rect);
        String::from_utf8_lossy(&bytes).into_owned()
    }

    // Returns the text within the rect as a String or panics if invalid utf8 bytes are found
    pub fn screen_get_text(&mut self, rect: &Rect) -> Result<String, ::std::string::FromUtf8Error> {
        let bytes = self.get_text_as_bytes(rect);
        let v = try! { String::from_utf8(bytes) };
        Ok(v)
    }

    fn get_text_as_bytes(&mut self, rect: &Rect) -> Vec<u8> {
        let size: usize = rect.size.width * rect.size.height;
        let mut text: Vec<c_char> = vec![0x0; size];
        let text_ptr: *mut c_char = (&mut text[0..size]).as_mut_ptr();
        unsafe {
            ffi::vterm_screen_get_text(self.screen_ptr.get(),
                                       text_ptr,
                                       text.len() as size_t,
                                       ffi::VTermRect::from_rect(&rect));
        }

        text.into_iter().map(|c| c as u8).collect()
    }

    pub fn screen_flush_damage(&mut self) {
        unsafe { ffi::vterm_screen_flush_damage(self.screen_ptr.get_mut()) };
    }

    // TODO: there should be a rust VTermDamageSize type for consistency
    pub fn screen_set_damage_merge(&mut self, size: ffi::VTermDamageSize) {
        unsafe { ffi::vterm_screen_set_damage_merge(self.screen_ptr.get_mut(), size) };
    }

    pub fn screen_get_cells_in_rect(&self, rect: &Rect) -> Vec<ScreenCell> {
        let mut cells: Vec<ScreenCell> = Vec::new(); // capacity is known here FYI

        for pos in rect.positions() {
            cells.push(self.screen_get_cell(&pos));
        }

        cells
    }

    /// calling this method will setup the vterm to generate ScreenEvent messages to a channel. The
    /// returned result indicates whether the channel was already created. The receiver end of the
    /// channel can be had by accessing the screen_events_rx field.
    pub fn generate_screen_events(&mut self) -> Result<(), ()> {
        if self.screen_callbacks_installed {
            return Err(());
        }
        self.screen_callbacks_installed = true;

        let (tx, rx) = mpsc::channel();
        self.screen_event_tx = Some(tx);
        self.screen_event_rx = Some(rx);

        unsafe {
            let self_ptr: *mut c_void = self as *mut _ as *mut c_void;
            ffi::vterm_screen_set_callbacks(self.screen_ptr.get_mut(),
                                            &::screen_callbacks::SCREEN_CALLBACKS,
                                            self_ptr);
        }

        Ok(())
    }
}

mod tests {
    #![allow(unused_imports)]
    use super::super::*;

    #[test]
    fn screen_can_reset() {
        let mut vterm: VTerm = VTerm::new(&Size {
            height: 2,
            width: 2,
        });
        vterm.screen_reset(true);
    }
}
