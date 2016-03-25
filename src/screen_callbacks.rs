use libc::{c_int, c_void};

use super::*;

pub extern "C" fn damage(rect: ffi::VTermRect, vterm: *mut c_void) -> c_int {
    let vterm: &mut VTerm = unsafe { &mut *(vterm as *mut VTerm) };
    match vterm.screen_event_tx.as_ref() {
        Some(tx) => {
            match tx.send(ScreenEvent::Damage(DamageEvent { rect: rect.as_rect() })) {
                Ok(_) => 1,
                Err(_) => 0,
            }
        }
        None => 0,
    }
}

pub extern "C" fn move_rect(dest: ffi::VTermRect,
                            src: ffi::VTermRect,
                            vterm: *mut c_void)
                            -> c_int {
    let vterm: &mut VTerm = unsafe { &mut *(vterm as *mut VTerm) };
    match vterm.screen_event_tx.as_ref() {
        Some(tx) => {
            match tx.send(ScreenEvent::MoveRect(MoveRectEvent {
                dest: dest.as_rect(),
                src: src.as_rect(),
            })) {
                Ok(_) => 1,
                Err(_) => 0,
            }
        }
        None => 0,
    }
}

pub extern "C" fn move_cursor(new: ffi::VTermPos,
                              old: ffi::VTermPos,
                              is_visible: c_int,
                              vterm: *mut c_void)
                              -> c_int {
    let vterm: &mut VTerm = unsafe { &mut *(vterm as *mut VTerm) };
    match vterm.screen_event_tx.as_ref() {
        Some(tx) => {
            let event = ScreenEvent::MoveCursor(MoveCursorEvent {
                new: new.as_pos(),
                old: old.as_pos(),
                is_visible: super::int_to_bool(is_visible),
            });
            match tx.send(event) {
                Ok(_) => 1,
                Err(_) => 0,
            }
        }
        None => 0,
    }
}

pub extern "C" fn set_term_prop(prop: ffi::VTermProp, value: ffi::VTermValue, vterm: *mut c_void) -> c_int {
    // I'm going to need a c shim to get a rust value out of the value union. And i think I'll need
    // to have the rust ffi::VTermValue type be the same size as the c union or the vterm pointer
    // may be pointing to the wrong memory?

    let event: ScreenEvent = match prop {
        ffi::VTermProp::VTermPropAltscreen => {
            ScreenEvent::AltScreen(AltScreenEvent { is_true: true })
        }
        ffi::VTermProp::VTermPropCursorblink => {
            ScreenEvent::CursorBlink(CursorBlinkEvent { is_true: true })
        }
        ffi::VTermProp::VTermPropCursorshape => {
            ScreenEvent::CursorShape(CursorShapeEvent { value: 0 })
        }
        ffi::VTermProp::VTermPropCursorvisible => {
            ScreenEvent::CursorVisible(CursorVisibleEvent { is_true: true })
        }
        ffi::VTermProp::VTermPropIconname => {
            ScreenEvent::IconName(IconNameEvent { text: "fake icon name".to_string() })
        }
        ffi::VTermProp::VTermPropMouse => ScreenEvent::Mouse(MouseEvent { value: 0 }),
        ffi::VTermProp::VTermPropReverse => ScreenEvent::Reverse(ReverseEvent { is_true: true }),
        ffi::VTermProp::VTermPropTitle => {
            ScreenEvent::Title(TitleEvent { text: "fake title".to_string() })
        }
    };

    info!("prop event {:?}", event);

    let vterm: &mut VTerm = unsafe { &mut *(vterm as *mut VTerm) };
    match vterm.screen_event_tx.as_ref() {
        Some(tx) => {
            match tx.send(event) {
                Ok(_) => 1,
                Err(_) => 0,
            }
        }
        None => 0,
    }
}

pub extern "C" fn bell(vterm: *mut c_void) -> c_int {
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
pub extern "C" fn resize(rows: c_int, cols: c_int, vterm: *mut c_void) -> c_int {
    let vterm: &mut VTerm = unsafe { &mut *(vterm as *mut VTerm) };
    match vterm.screen_event_tx.as_ref() {
        Some(tx) => {
            match tx.send(ScreenEvent::Resize(ResizeEvent {
                height: rows as usize,
                width: cols as usize,
            })) {
                Ok(_) => 1,
                Err(_) => 0,
            }
        }
        None => 0,
    }
}
pub extern "C" fn sb_pushline(cols: c_int,
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

            match tx.send(ScreenEvent::SbPushLine(SbPushLineEvent { cells: cells })) {
                Ok(_) => 1,
                Err(_) => 0,
            }
        }
        None => 0,
    }
}

pub extern "C" fn sb_popline(cols: c_int,
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

            match tx.send(ScreenEvent::SbPopLine(SbPopLineEvent { cells: cells })) {
                Ok(_) => 1,
                Err(_) => 0,
            }
        }
        None => 0,
    }
}
