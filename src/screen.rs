extern crate libc;

use libc::{c_int, c_void};
use std::sync::mpsc;

use super::*;

pub struct ScreenSize {
    pub rows: usize,
    pub cols: usize,
}

#[derive(Debug)]
pub struct Pos {
    pub row: usize,
    pub col: usize,
}

#[derive(Debug)]
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
    MoveCursor  { pos: Pos,       old_pos: Pos,    is_visible: bool },
    //SetTermProp { prop: Prop,     value: Value },
    Bell,
    Resize      { rows: usize,         cols: usize },
    SbPushLine  { cells: Vec<Cell> },
    SbPopLine   { cells: Vec<Cell> },
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

            tx.send(ScreenEvent::Damage { rect: rust_rect });
            1
        },
        None => 0
    }
}

extern "C" fn move_rect_handler(dest: ffi::VTermRect, src: ffi::VTermRect, tx: *mut c_void) -> c_int { 1 }
extern "C" fn move_cursor_handler(pos: ffi::VTermPos, old_pos: ffi::VTermPos, is_visible: c_int, tx: *mut c_void) -> c_int { 1 }
extern "C" fn set_term_prop_handler(_: ffi::VTermProp, _: ffi::VTermValue, tx: *mut c_void) -> c_int { 1 }
extern "C" fn bell_handler(tx: *mut c_void) -> c_int { 1 }
extern "C" fn resize_handler(rows: c_int, cols: c_int, tx: *mut c_void) -> c_int {
    let tx: &mut Option<mpsc::Sender<ScreenEvent>> = unsafe { &mut *(tx as *mut Option<mpsc::Sender<ScreenEvent>>) };
    match tx.as_ref() {
        Some(tx) => {
            tx.send(ScreenEvent::Resize { rows: rows as usize, cols: cols as usize} );
            1
        },
        None => 0
    }
}
extern "C" fn sb_pushline_handler(cols: c_int, cells: *const ffi::VTermScreenCell, tx: *mut c_void) -> c_int { 1 }
extern "C" fn sb_popline_handler(cols: c_int, cells: *const ffi::VTermScreenCell, tx: *mut c_void) -> c_int { 1 }

pub static screen_callbacks: ffi::VTermScreenCallbacks = ffi::VTermScreenCallbacks {
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
    pub fn from_ptr(ptr: *mut ffi::VTermScreen) -> Screen {
        Screen { ptr: ptr }
    }

    pub fn reset(&mut self, is_hard: bool) {
        unsafe { ffi::vterm_screen_reset(self.ptr, super::bool_to_int(is_hard)) }
    }

    pub fn get_cell(&self, pos: &Pos) -> Cell {
        let pos = ffi::VTermPos { row: pos.row as c_int, col: pos.col as c_int };
        let cell_ptr = unsafe { ffi::vterm_cell_new() };
        unsafe { ffi::vterm_screen_get_cell(self.ptr, pos, cell_ptr) };
        Cell::from_ptr(cell_ptr)
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
