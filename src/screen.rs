extern crate libc;

use libc::{c_int, c_void};
use std::sync::mpsc;

use super::*;

#[derive(Debug, Default)]
pub struct ScreenSize {
    pub rows: u16,
    pub cols: u16,
}

#[derive(Debug, Default, Clone)]
pub struct Pos {
    /// negative numbers represent scroll buffer positions
    pub row: i16,
    pub col: i16,
}

#[derive(Debug, Default)]
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

extern "C" fn damage_handler(rect: ffi::VTermRect, tx: *mut c_void) -> c_int {
    let tx: &mut Option<mpsc::Sender<ScreenEvent>> = unsafe { &mut *(tx as *mut Option<mpsc::Sender<ScreenEvent>>) };
    match tx.as_ref() {
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

extern "C" fn move_rect_handler(dest: ffi::VTermRect, src: ffi::VTermRect, tx: *mut c_void) -> c_int {
    let tx: &mut Option<mpsc::Sender<ScreenEvent>> = unsafe { &mut *(tx as *mut Option<mpsc::Sender<ScreenEvent>>) };
    match tx.as_ref() {
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

extern "C" fn move_cursor_handler(new: ffi::VTermPos, old: ffi::VTermPos, is_visible: c_int, tx: *mut c_void) -> c_int {
    let tx: &mut Option<mpsc::Sender<ScreenEvent>> = unsafe { &mut *(tx as *mut Option<mpsc::Sender<ScreenEvent>>) };
    match tx.as_ref() {
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

extern "C" fn set_term_prop_handler(prop: ffi::VTermProp, value: ffi::VTermValue, tx: *mut c_void) -> c_int {
    let tx: &mut Option<mpsc::Sender<ScreenEvent>> = unsafe { &mut *(tx as *mut Option<mpsc::Sender<ScreenEvent>>) };

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
    0

    // This crashes inside the channel somewhere. Don't know why.
    //match tx.as_ref() {
        //Some(tx) => {
            //match tx.send(event) {
                //Ok(_) => 1,
                //Err(_) => 0,
            //}
        //},
        //None => 0
    //}
}

extern "C" fn bell_handler(tx: *mut c_void) -> c_int {
    let tx: &mut Option<mpsc::Sender<ScreenEvent>> = unsafe { &mut *(tx as *mut Option<mpsc::Sender<ScreenEvent>>) };
    match tx.as_ref() {
        Some(tx) => {
            match tx.send(ScreenEvent::Bell) {
                Ok(_) => 1,
                Err(_) => 0,
            }
        },
        None => 0
    }
}
extern "C" fn resize_handler(rows: c_int, cols: c_int, tx: *mut c_void) -> c_int {
    let tx: &mut Option<mpsc::Sender<ScreenEvent>> = unsafe { &mut *(tx as *mut Option<mpsc::Sender<ScreenEvent>>) };
    match tx.as_ref() {
        Some(tx) => {
            match tx.send(ScreenEvent::Resize { rows: rows as u16, cols: cols as u16} ) {
                Ok(_) => 1,
                Err(_) => 0,
            }
        },
        None => 0
    }
}
extern "C" fn sb_pushline_handler(cols: c_int, cells_ptr: *const ffi::VTermScreenCell, tx: *mut c_void) -> c_int {
    let tx: &mut Option<mpsc::Sender<ScreenEvent>> = unsafe { &mut *(tx as *mut Option<mpsc::Sender<ScreenEvent>>) };
    match tx.as_ref() {
        Some(tx) => {
            let mut cells = vec!();
            for i in 0..(cols as isize) {
                let ptr = unsafe { ffi::vterm_cell_pointer_arithmetic(cells_ptr, i as c_int) };
                cells.push(ScreenCell::from_ptr(ptr, Pos { row: -1, col: -1}));
            }

            match tx.send(ScreenEvent::SbPushLine { cells: cells }) {
                Ok(_) => 1,
                Err(_) => 0,
            }
        },
        None => 0
    }
}

extern "C" fn sb_popline_handler(cols: c_int, cells_ptr: *const ffi::VTermScreenCell, tx: *mut c_void) -> c_int {
    let tx: &mut Option<mpsc::Sender<ScreenEvent>> = unsafe { &mut *(tx as *mut Option<mpsc::Sender<ScreenEvent>>) };
    match tx.as_ref() {
        Some(tx) => {
            let mut cells = vec!();
            for i in 0..(cols as isize) {
                let ptr = unsafe { ffi::vterm_cell_pointer_arithmetic(cells_ptr, i as c_int) };
                cells.push(ScreenCell::from_ptr(ptr, Pos { row: -1, col: -1}));
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

    /// Return the cell at the given position
    pub fn get_cell(&self, pos: &Pos) -> ScreenCell {
        let ffi_pos = ffi::VTermPos { row: pos.row as c_int, col: pos.col as c_int };
        let cell_buf = unsafe { ffi::vterm_cell_new() };
        unsafe { ffi::vterm_screen_get_cell(self.ptr, ffi_pos, cell_buf) };
        let cell = ScreenCell::from_ptr(cell_buf, pos.clone());
        unsafe { ffi::vterm_cell_free(cell_buf) };

        cell
    }

    pub fn flush_damage(&mut self) {
        unsafe { ffi::vterm_screen_flush_damage(self.ptr) };
    }
}

mod tests {
    use super::super::*;

    #[test]
    fn screen_can_reset() {
        let vterm: VTerm = VTerm::new(2, 2);
        let mut screen = vterm.get_screen();
        screen.reset(true);
    }
}
