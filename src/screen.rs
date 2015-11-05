extern crate libc;

use libc::{c_int, c_void};
use std::sync::mpsc;

use super::*;

#[derive(Debug, Default)]
pub struct ScreenSize {
    pub rows: usize,
    pub cols: usize,
}

#[derive(Debug, Default)]
pub struct Pos {
    pub row: usize,
    pub col: usize,
}

#[derive(Debug, Default)]
pub struct Rect {
    pub start_row: usize,
    pub end_row: usize,
    pub start_col: usize,
    pub end_col: usize,
}

#[derive(Debug)]
pub enum ScreenEvent {
    Damage      { rect: Rect },
    MoveRect    { dest: Rect,     src: Rect },
    MoveCursor  { new: Pos,       old: Pos,    is_visible: bool },
    //SetTermProp { prop: Prop,     value: Value },
    Bell,
    Resize      { rows: usize,         cols: usize },
    SbPushLine  { cells: Vec<ScreenCell> },
    SbPopLine   { cells: Vec<ScreenCell> },
}

extern "C" fn damage_handler(rect: ffi::VTermRect, tx: *mut c_void) -> c_int {
    let tx: &mut Option<mpsc::Sender<ScreenEvent>> = unsafe { &mut *(tx as *mut Option<mpsc::Sender<ScreenEvent>>) };
    match tx.as_ref() {
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
        },
        None => 0
    }
}

extern "C" fn move_rect_handler(dest: ffi::VTermRect, src: ffi::VTermRect, tx: *mut c_void) -> c_int {
    let tx: &mut Option<mpsc::Sender<ScreenEvent>> = unsafe { &mut *(tx as *mut Option<mpsc::Sender<ScreenEvent>>) };
    match tx.as_ref() {
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
            let rust_new = Pos { row: new.row as usize, col: new.col as usize };
            let rust_old = Pos { row: old.row as usize, col: old.col as usize };
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
extern "C" fn set_term_prop_handler(_: ffi::VTermProp, _: ffi::VTermValue, tx: *mut c_void) -> c_int { 0 }
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
            match tx.send(ScreenEvent::Resize { rows: rows as usize, cols: cols as usize} ) {
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
                cells.push(ScreenCell::from_ptr(ptr));
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
                cells.push(ScreenCell::from_ptr(ptr));
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
        let pos = ffi::VTermPos { row: pos.row as c_int, col: pos.col as c_int };
        let cell_buf = unsafe { ffi::vterm_cell_new() };
        unsafe { ffi::vterm_screen_get_cell(self.ptr, pos, cell_buf) };
        let cell = ScreenCell::from_ptr(cell_buf);
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
