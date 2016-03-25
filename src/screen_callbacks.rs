use libc::{c_int, c_void};

use super::*;

extern "C" fn damage_handler(rect: ffi::VTermRect, vterm: *mut c_void) -> c_int {
    let vterm: &mut VTerm = unsafe { &mut *(vterm as *mut VTerm) };
    match vterm.screen_event_tx.as_ref() {
        Some(tx) => {
            match tx.send(ScreenEvent::Damage { rect: rect.as_rect() }) {
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
            match tx.send(ScreenEvent::MoveRect {
                dest: dest.as_rect(),
                src: src.as_rect(),
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
            let event = ScreenEvent::MoveCursor {
                new: new.as_pos(),
                old: old.as_pos(),
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
                height: rows as usize,
                width: cols as usize,
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
