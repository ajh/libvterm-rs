use libc::{c_int, c_void, size_t, c_char};

use super::*;

extern "C" fn damage_handler(rect: ffi::VTermRect, vterm: *mut c_void) -> c_int {
    let vterm: &mut VTerm = unsafe { &mut *(vterm as *mut VTerm) };
    match vterm.screen_event_tx.as_ref() {
        Some(tx) => {
            let rust_rect = Rect {
                start_row: rect.start_row as usize,
                end_row: rect.end_row as usize,
                start_col: rect.start_col as usize,
                end_col: rect.end_col as usize,
            };

            match tx.send(ScreenEvent::Damage { rect: rust_rect }) {
                Ok(_) => 1,
                Err(_) => 0,
            }
        }
        None => 0,
    }
}

extern "C" fn move_rect_handler(dest: ffi::VTermRect,
                                src: ffi::VTermRect,
                                vterm: *mut c_void)
                                -> c_int {
    let vterm: &mut VTerm = unsafe { &mut *(vterm as *mut VTerm) };
    match vterm.screen_event_tx.as_ref() {
        Some(tx) => {
            let rust_dest = Rect {
                start_row: dest.start_row as usize,
                end_row: dest.end_row as usize,
                start_col: dest.start_col as usize,
                end_col: dest.end_col as usize,
            };

            let rust_src = Rect {
                start_row: src.start_row as usize,
                end_row: src.end_row as usize,
                start_col: src.start_col as usize,
                end_col: src.end_col as usize,
            };

            match tx.send(ScreenEvent::MoveRect {
                dest: rust_dest,
                src: rust_src,
            }) {
                Ok(_) => 1,
                Err(_) => 0,
            }
        }
        None => 0,
    }
}

extern "C" fn move_cursor_handler(new: ffi::VTermPos,
                                  old: ffi::VTermPos,
                                  is_visible: c_int,
                                  vterm: *mut c_void)
                                  -> c_int {
    let vterm: &mut VTerm = unsafe { &mut *(vterm as *mut VTerm) };
    match vterm.screen_event_tx.as_ref() {
        Some(tx) => {
            let rust_new = Pos {
                row: new.row as usize,
                col: new.col as usize,
            };
            let rust_old = Pos {
                row: old.row as usize,
                col: old.col as usize,
            };
            let event = ScreenEvent::MoveCursor {
                new: rust_new,
                old: rust_old,
                is_visible: super::int_to_bool(is_visible),
            };
            match tx.send(event) {
                Ok(_) => 1,
                Err(_) => 0,
            }
        }
        None => 0,
    }
}

extern "C" fn set_term_prop_handler(_: ffi::VTermProp,
                                    _: ffi::VTermValue,
                                    _: *mut c_void)
                                    -> c_int {
    return 0;

    // This crashes inside the channel somewhere. Don't know why.
    // let event: ScreenEvent = match prop {
    // ffi::VTermProp::VTermPropAltscreen     => ScreenEvent::AltScreen     { is_true: true },
    // ffi::VTermProp::VTermPropCursorblink   => ScreenEvent::CursorBlink   { is_true: true },
    // ffi::VTermProp::VTermPropCursorshape   => ScreenEvent::CursorShape   { value: 0 },
    // ffi::VTermProp::VTermPropCursorvisible => ScreenEvent::CursorVisible { is_true: true },
    // ffi::VTermProp::VTermPropIconname      => ScreenEvent::IconName      { text: "fake icon name".to_string() },
    // ffi::VTermProp::VTermPropMouse         => ScreenEvent::Mouse         { value: 0 },
    // ffi::VTermProp::VTermPropReverse       => ScreenEvent::Reverse       { is_true: true },
    // ffi::VTermProp::VTermPropTitle         => ScreenEvent::Title         { text: "fake title".to_string() },
    // };

    // info!("prop event {:?}", event);

    // let vterm: &mut VTerm = unsafe { &mut *(vterm as *mut VTerm) };
    // match vterm.screen_event_tx.as_ref() {
    // Some(tx) => {
    // match tx.send(event) {
    // Ok(_) => 1,
    // Err(_) => 0,
    // }
    // },
    // None => 0
    // }
}

extern "C" fn bell_handler(vterm: *mut c_void) -> c_int {
    let vterm: &mut VTerm = unsafe { &mut *(vterm as *mut VTerm) };
    match vterm.screen_event_tx.as_ref() {
        Some(tx) => {
            match tx.send(ScreenEvent::Bell) {
                Ok(_) => 1,
                Err(_) => 0,
            }
        }
        None => 0,
    }
}
extern "C" fn resize_handler(rows: c_int, cols: c_int, vterm: *mut c_void) -> c_int {
    let vterm: &mut VTerm = unsafe { &mut *(vterm as *mut VTerm) };
    match vterm.screen_event_tx.as_ref() {
        Some(tx) => {
            match tx.send(ScreenEvent::Resize {
                rows: rows as usize,
                cols: cols as usize,
            }) {
                Ok(_) => 1,
                Err(_) => 0,
            }
        }
        None => 0,
    }
}
extern "C" fn sb_pushline_handler(cols: c_int,
                                  cells_ptr: *const ffi::VTermScreenCell,
                                  vterm: *mut c_void)
                                  -> c_int {
    let vterm: &mut VTerm = unsafe { &mut *(vterm as *mut VTerm) };
    match vterm.screen_event_tx.as_ref() {
        Some(tx) => {
            let mut cells = vec![];
            for i in 0..(cols as usize) {
                let ptr = unsafe { ffi::vterm_cell_pointer_arithmetic(cells_ptr, i as c_int) };
                cells.push(ScreenCell::from_ptr(ptr, &vterm));
            }

            match tx.send(ScreenEvent::SbPushLine { cells: cells }) {
                Ok(_) => 1,
                Err(_) => 0,
            }
        }
        None => 0,
    }
}

extern "C" fn sb_popline_handler(cols: c_int,
                                 cells_ptr: *const ffi::VTermScreenCell,
                                 vterm: *mut c_void)
                                 -> c_int {
    let vterm: &mut VTerm = unsafe { &mut *(vterm as *mut VTerm) };
    match vterm.screen_event_tx.as_ref() {
        Some(tx) => {
            let mut cells = vec![];
            for i in 0..(cols as usize) {
                let ptr = unsafe { ffi::vterm_cell_pointer_arithmetic(cells_ptr, i as c_int) };
                cells.push(ScreenCell::from_ptr(ptr, &vterm));
            }

            match tx.send(ScreenEvent::SbPopLine { cells: cells }) {
                Ok(_) => 1,
                Err(_) => 0,
            }
        }
        None => 0,
    }
}

pub static SCREEN_CALLBACKS: ffi::VTermScreenCallbacks = ffi::VTermScreenCallbacks {
    damage: damage_handler,
    move_rect: move_rect_handler,
    move_cursor: move_cursor_handler,
    set_term_prop: set_term_prop_handler,
    bell: bell_handler,
    resize: resize_handler,
    sb_pushline: sb_pushline_handler,
    sb_popline: sb_popline_handler,
};

impl VTerm {
    /// Reset the screen. I've observed this needs to happen before using or segfaults will occur.
    pub fn screen_reset(&mut self, is_hard: bool) {
        unsafe { ffi::vterm_screen_reset(self.screen_ptr.get_mut(), super::bool_to_int(is_hard)) }
    }

    /// Return the cell at the given position
    pub fn screen_get_cell(&self, pos: &Pos) -> ScreenCell {
        let ffi_pos = ffi::VTermPos {
            row: pos.row as c_int,
            col: pos.col as c_int,
        };
        let cell_buf = unsafe { ffi::vterm_cell_new() };
        unsafe { ffi::vterm_screen_get_cell(self.screen_ptr.get(), ffi_pos, cell_buf) };
        let cell = ScreenCell::from_ptr(cell_buf, &self); // shouldn't this take &cell_buf?
        unsafe { ffi::vterm_cell_free(cell_buf) };

        cell
    }

    // It seems wrong to be converting the u8's to rust chars here since its lossy. Better to leave
    // that decision to the caller.
    pub fn screen_get_text(&mut self, rect: &Rect) -> String {
        let size: usize = ((rect.end_row - rect.start_row + 1) *
                           (rect.end_col - rect.start_col + 1)) as usize;
        let mut text: Vec<c_char> = vec![0x0; size];
        let rect = ffi::VTermRect {
            start_row: rect.start_row as i32,
            end_row: rect.end_row as i32,
            start_col: rect.start_col as i32,
            end_col: rect.end_col as i32,
        };
        let text_ptr: *mut c_char = (&mut text[0..size]).as_mut_ptr();
        unsafe {
            ffi::vterm_screen_get_text(self.screen_ptr.get(), text_ptr, text.len() as size_t, rect);
        }

        let text: Vec<u8> = text.into_iter().map(|c| c as u8).collect();
        String::from_utf8_lossy(&text).into_owned()
    }

    pub fn screen_flush_damage(&mut self) {
        unsafe { ffi::vterm_screen_flush_damage(self.screen_ptr.get_mut()) };
    }

    // TODO: there should be a rust VTermDamageSize type for consistency
    pub fn screen_set_damage_merge(&mut self, size: ffi::VTermDamageSize) {
        unsafe { ffi::vterm_screen_set_damage_merge(self.screen_ptr.get_mut(), size) };
    }

    pub fn screen_get_cells_in_rect(&self, rect: &Rect) -> Vec<ScreenCell> {
        let mut pos: Pos = Default::default();
        let mut cells: Vec<ScreenCell> = Vec::new(); // capacity is known here FYI

        for row in rect.start_row..rect.end_row {
            pos.row = row as usize;
            for col in rect.start_col..rect.end_col {
                pos.col = col as usize;
                cells.push(self.screen_get_cell(&pos));
            }
        }

        cells
    }
}

mod tests {
    #![allow(unused_imports)]
    use super::super::*;

    #[test]
    fn screen_can_reset() {
        let mut vterm: VTerm = VTerm::new(&ScreenSize { rows: 2, cols: 2 });
        vterm.screen_reset(true);
    }
}
