use libc::{c_int, c_void, size_t, c_char};

use super::*;

#[derive(Debug, Default, Clone, PartialEq)]
pub struct ScreenSize {
    pub rows: u16,
    pub cols: u16,
}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct Pos {
    /// negative numbers represent scroll buffer positions
    pub row: i16,
    pub col: i16,
}

#[derive(Debug, Default, PartialEq)]
pub struct Rect {
    pub start_row: u16,
    pub end_row: u16,
    pub start_col: u16,
    pub end_col: u16,
}

#[derive(Debug)]
pub enum ScreenEvent {
    AltScreen     { is_true: bool },
    Bell,
    CursorBlink   { is_true: bool },
    CursorShape   { value: isize },
    CursorVisible { is_true: bool },
    Damage        { rect: Rect },
    IconName      { text: String},
    Mouse         { value: isize },
    MoveCursor    { new: Pos,                  old: Pos,       is_visible: bool },
    MoveRect      { dest: Rect,                src: Rect },
    Resize        { rows: u16,               cols: u16 },
    Reverse       { is_true: bool },
    SbPopLine     { cells: Vec<ScreenCell> },
    SbPushLine    { cells: Vec<ScreenCell> },
    Title         { text: String},
}

extern "C" fn damage_handler(rect: ffi::VTermRect, vterm: *mut c_void) -> c_int {
    let vterm: &mut VTerm = unsafe { &mut *(vterm as *mut VTerm) };
    match vterm.screen_event_tx.as_ref() {
        Some(tx) => {
            let rust_rect = Rect {
                start_row: rect.start_row as u16,
                end_row: rect.end_row as u16,
                start_col: rect.start_col as u16,
                end_col: rect.end_col as u16,
            };

            match tx.send(ScreenEvent::Damage { rect: rust_rect }) {
                Ok(_) => 1,
                Err(_) => 0,
            }
        },
        None => 0
    }
}

extern "C" fn move_rect_handler(dest: ffi::VTermRect, src: ffi::VTermRect, vterm: *mut c_void) -> c_int {
    let vterm: &mut VTerm = unsafe { &mut *(vterm as *mut VTerm) };
    match vterm.screen_event_tx.as_ref() {
        Some(tx) => {
            let rust_dest = Rect {
                start_row: dest.start_row as u16,
                end_row: dest.end_row as u16,
                start_col: dest.start_col as u16,
                end_col: dest.end_col as u16,
            };

            let rust_src = Rect {
                start_row: src.start_row as u16,
                end_row: src.end_row as u16,
                start_col: src.start_col as u16,
                end_col: src.end_col as u16,
            };

            match tx.send(ScreenEvent::MoveRect { dest: rust_dest, src: rust_src }) {
                Ok(_) => 1,
                Err(_) => 0,
            }
        },
        None => 0
    }
}

extern "C" fn move_cursor_handler(new: ffi::VTermPos, old: ffi::VTermPos, is_visible: c_int, vterm: *mut c_void) -> c_int {
    let vterm: &mut VTerm = unsafe { &mut *(vterm as *mut VTerm) };
    match vterm.screen_event_tx.as_ref() {
        Some(tx) => {
            let rust_new = Pos { row: new.row as i16, col: new.col as i16 };
            let rust_old = Pos { row: old.row as i16, col: old.col as i16 };
            let event = ScreenEvent::MoveCursor {
                new: rust_new,
                old: rust_old,
                is_visible: super::int_to_bool(is_visible)
            };
            match tx.send(event) {
                Ok(_) => 1,
                Err(_) => 0,
            }
        },
        None => 0
    }
}

extern "C" fn set_term_prop_handler(prop: ffi::VTermProp, _: ffi::VTermValue, vterm: *mut c_void) -> c_int {
    let event: ScreenEvent = match prop {
        ffi::VTermProp::VTermPropAltscreen     => ScreenEvent::AltScreen     { is_true: true },
        ffi::VTermProp::VTermPropCursorblink   => ScreenEvent::CursorBlink   { is_true: true },
        ffi::VTermProp::VTermPropCursorshape   => ScreenEvent::CursorShape   { value: -1 },
        ffi::VTermProp::VTermPropCursorvisible => ScreenEvent::CursorVisible { is_true: true },
        ffi::VTermProp::VTermPropIconname      => ScreenEvent::IconName      { text: "fake icon name".to_string() },
        ffi::VTermProp::VTermPropMouse         => ScreenEvent::Mouse         { value: -1 },
        ffi::VTermProp::VTermPropReverse       => ScreenEvent::Reverse       { is_true: true },
        ffi::VTermProp::VTermPropTitle         => ScreenEvent::Title         { text: "fake title".to_string() },
    };

    info!("prop event {:?}", event);

    // This crashes inside the channel somewhere. Don't know why.
    let vterm: &mut VTerm = unsafe { &mut *(vterm as *mut VTerm) };
    match vterm.screen_event_tx.as_ref() {
        Some(tx) => {
            match tx.send(event) {
                Ok(_) => 1,
                Err(_) => 0,
            }
        },
        None => 0
    }
}

extern "C" fn bell_handler(vterm: *mut c_void) -> c_int {
    let vterm: &mut VTerm = unsafe { &mut *(vterm as *mut VTerm) };
    match vterm.screen_event_tx.as_ref() {
        Some(tx) => {
            match tx.send(ScreenEvent::Bell) {
                Ok(_) => 1,
                Err(_) => 0,
            }
        },
        None => 0
    }
}
extern "C" fn resize_handler(rows: c_int, cols: c_int, vterm: *mut c_void) -> c_int {
    let vterm: &mut VTerm = unsafe { &mut *(vterm as *mut VTerm) };
    match vterm.screen_event_tx.as_ref() {
        Some(tx) => {
            match tx.send(ScreenEvent::Resize { rows: rows as u16, cols: cols as u16} ) {
                Ok(_) => 1,
                Err(_) => 0,
            }
        },
        None => 0
    }
}
extern "C" fn sb_pushline_handler(cols: c_int, cells_ptr: *const ffi::VTermScreenCell, vterm: *mut c_void) -> c_int {
    let vterm: &mut VTerm = unsafe { &mut *(vterm as *mut VTerm) };
    match vterm.screen_event_tx.as_ref() {
        Some(tx) => {
            let mut cells = vec!();
            for i in 0..(cols as isize) {
                let ptr = unsafe { ffi::vterm_cell_pointer_arithmetic(cells_ptr, i as c_int) };
                cells.push(ScreenCell::from_ptr(ptr, Pos { row: -1, col: -1}, &vterm.state));
            }

            match tx.send(ScreenEvent::SbPushLine { cells: cells }) {
                Ok(_) => 1,
                Err(_) => 0,
            }
        },
        None => 0
    }
}

extern "C" fn sb_popline_handler(cols: c_int, cells_ptr: *const ffi::VTermScreenCell, vterm: *mut c_void) -> c_int {
    let vterm: &mut VTerm = unsafe { &mut *(vterm as *mut VTerm) };
    match vterm.screen_event_tx.as_ref() {
        Some(tx) => {
            let mut cells = vec!();
            for i in 0..(cols as isize) {
                let ptr = unsafe { ffi::vterm_cell_pointer_arithmetic(cells_ptr, i as c_int) };
                cells.push(ScreenCell::from_ptr(ptr, Pos { row: -1, col: -1}, &vterm.state));
            }

            match tx.send(ScreenEvent::SbPopLine { cells: cells }) {
                Ok(_) => 1,
                Err(_) => 0,
            }
        },
        None => 0
    }
}

pub static SCREEN_CALLBACKS: ffi::VTermScreenCallbacks = ffi::VTermScreenCallbacks {
    damage:             damage_handler,
    move_rect:          move_rect_handler,
    move_cursor:        move_cursor_handler,
    set_term_prop:      set_term_prop_handler,
    bell:               bell_handler,
    resize:             resize_handler,
    sb_pushline:        sb_pushline_handler,
    sb_popline:         sb_popline_handler,
};

pub struct Screen {
    ptr: *mut ffi::VTermScreen
}

impl Screen {
    /// Create a new Screen from a pointer. This pointer will not get free'ed because the vterm
    /// handles that.
    pub fn from_ptr(ptr: *mut ffi::VTermScreen) -> Screen {
        Screen { ptr: ptr }
    }

    /// Reset the screen. I've observed this needs to happen before using or segfaults will occur.
    pub fn reset(&mut self, is_hard: bool) {
        unsafe { ffi::vterm_screen_reset(self.ptr, super::bool_to_int(is_hard)) }
    }

    /// Return the cell at the given position. Use the method on VTerm please.
    pub fn get_cell(&self, pos: &Pos, state: &State) -> ScreenCell {
        let ffi_pos = ffi::VTermPos { row: pos.row as c_int, col: pos.col as c_int };
        let cell_buf = unsafe { ffi::vterm_cell_new() };
        unsafe { ffi::vterm_screen_get_cell(self.ptr, ffi_pos, cell_buf) };
        let cell = ScreenCell::from_ptr(cell_buf, pos.clone(), state); // shouldn't this take &cell_buf?
        unsafe { ffi::vterm_cell_free(cell_buf) };

        cell
    }

    pub fn get_text(&mut self, rect: Rect) -> String {
        let size: usize = ((rect.end_row - rect.start_row + 1) * (rect.end_col - rect.start_col + 1)) as usize;
        let mut text: Vec<c_char> = vec![0x0; size];
        let rect = ffi::VTermRect { start_row: rect.start_row as i32,
                                    end_row: rect.end_row as i32,
                                    start_col: rect.start_col as i32,
                                    end_col: rect.end_col as i32 };
        let text_ptr: *mut c_char = (&mut text[0..size]).as_mut_ptr();
        unsafe { ffi::vterm_screen_get_text(self.ptr, text_ptr, text.len() as size_t, rect); }

        let text: Vec<u8> = text.into_iter().map( |c| c as u8 ).collect();
        String::from_utf8_lossy(&text).into_owned()
    }

    pub fn flush_damage(&mut self) {
        unsafe { ffi::vterm_screen_flush_damage(self.ptr) };
    }

    pub fn set_damage_merge(&mut self, size: ffi::VTermDamageSize) {
        unsafe { ffi::vterm_screen_set_damage_merge(self.ptr, size) };
    }
}

mod tests {
    #[test]
    fn screen_can_reset() {
        let mut vterm: ::VTerm = ::VTerm::new(::ScreenSize { rows: 2, cols: 2 });
        vterm.screen.reset(true);
    }
}
