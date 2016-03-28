#![allow(unused_variables)]

use libc::{c_int, c_void};
use std::sync::mpsc::Sender;
use std::ffi::CString;

use super::*;

// int (*putglyph)(VTermGlyphInfo *info, VTermPos pos, void *user);
pub extern "C" fn put_glyph(info: *mut ffi::VTermGlyphInfo,
                        pos: ffi::VTermPos,
                        vterm: *mut c_void)
                        -> c_int {
    with_sender(vterm, |tx| {
        let event = StateEvent::PutGlyph(PutGlyphEvent {
            glyph_info: ::GlyphInfo::from_ptr(info),
            pos: pos.as_pos(),
        });

        match tx.send(event) {
            Ok(_) => 1,
            Err(_) => 0,
        }
    })
}

// int (*movecursor)(VTermPos pos, VTermPos oldpos, int visible, void *user);
pub extern "C" fn move_cursor(new: ffi::VTermPos,
                          old: ffi::VTermPos,
                          visible: c_int,
                          vterm: *mut c_void)
                          -> c_int {
    with_sender(vterm, |tx| {
        let event = StateEvent::MoveCursor(MoveCursorEvent {
            new: new.as_pos(),
            old: old.as_pos(),
            is_visible: int_to_bool(visible),
        });

        match tx.send(event) {
            Ok(_) => 1,
            Err(_) => 0,
        }
    })
}

// int (*scrollrect)(VTermRect rect, int downward, int rightward, void *user);
pub extern "C" fn scroll_rect(rect: ffi::VTermRect,
                          downward: c_int,
                          rightward: c_int,
                          vterm: *mut c_void)
                          -> c_int {
    with_sender(vterm, |tx| {
        let event = StateEvent::ScrollRect(ScrollRectEvent {
            rect: rect.as_rect(),
            downward: downward as isize,
            rightward: rightward as isize,
        });

        match tx.send(event) {
            Ok(_) => 1,
            Err(_) => 0,
        }
    })
}

// int (*moverect)(VTermRect dest, VTermRect src, void *user);
pub extern "C" fn move_rect(dest: ffi::VTermRect, src: ffi::VTermRect, vterm: *mut c_void) -> c_int {
    0
}
// int (*erase)(VTermRect rect, int selective, void *user);
pub extern "C" fn erase(rect: ffi::VTermRect, selective: c_int, vterm: *mut c_void) -> c_int {
    0
}
// int (*initpen)(void *user);
pub extern "C" fn init_pen(vterm: *mut c_void) -> c_int {
    0
}
// int (*setpenattr)(VTermAttr attr, VTermValue *val, void *user);
pub extern "C" fn set_pen_attr(attr: ffi::VTermAttr,
                           val: *mut ffi::VTermValue,
                           vterm: *mut c_void)
                           -> c_int {
    0
}
// int (*settermprop)(VTermProp prop, VTermValue *val, void *user);
pub extern "C" fn set_term_prop(prop: ffi::VTermProp,
                            val: *mut ffi::VTermValue,
                            vterm: *mut c_void)
                            -> c_int {
    println!("prop is {:?}", prop);
    with_sender(vterm, |tx| {
        let event: StateEvent = match prop {
            ffi::VTermProp::VTermPropCursorvisible => {
                let val = unsafe { int_to_bool(ffi::vterm_value_get_boolean(val)) };
                StateEvent::CursorVisible(CursorVisibleEvent { is_true: val })
            }

            ffi::VTermProp::VTermPropAltscreen => {
                let val = unsafe { int_to_bool(ffi::vterm_value_get_boolean(val)) };
                StateEvent::AltScreen(AltScreenEvent { is_true: val })
            }

            ffi::VTermProp::VTermPropCursorblink => {
                let val = unsafe { int_to_bool(ffi::vterm_value_get_boolean(val)) };
                StateEvent::CursorBlink(CursorBlinkEvent { is_true: val })
            }

            ffi::VTermProp::VTermPropCursorshape => {
                let val = unsafe { ffi::vterm_value_get_number(val) };
                StateEvent::CursorShape(CursorShapeEvent { value: val })
            }

            //ffi::VTermProp::VTermPropIconname => {
                //StateEvent::IconName(IconNameEvent { text: "fake icon name".to_string() })
            //}

            ffi::VTermProp::VTermPropMouse => {
                let val = unsafe { ffi::vterm_value_get_number(val) };
                StateEvent::Mouse(MouseEvent { value: val })
            },

            //ffi::VTermProp::VTermPropReverse => StateEvent::Reverse(ReverseEvent { is_true: true }),
            ffi::VTermProp::VTermPropTitle => {
                let val = unsafe { CString::from_raw(ffi::vterm_value_get_string(val)).into_string().unwrap() };
                StateEvent::Title(TitleEvent { text: val })
            }

            // This is wrong: FIXME
            _ => { StateEvent::Bell },
        };

        match tx.send(event) {
            Ok(_) => 1,
            Err(_) => 0,
        }
    })
}

// int (*bell)(void *user);
pub extern "C" fn bell(vterm: *mut c_void) -> c_int {
    0
}
// int (*resize)(int rows, int cols, VTermPos *delta, void *user);
pub extern "C" fn resize(rows: c_int,
                     cols: c_int,
                     delta: *mut ffi::VTermPos,
                     vterm: *mut c_void)
                     -> c_int {
    0
}
// int (*setlineinfo)(int row, const VTermLineInfo *newinfo, const VTermLineInfo *oldinfo, void *user);
pub extern "C" fn set_line_info(row: c_int,
                            new: *const ffi::VTermLineInfo,
                            old: *const ffi::VTermLineInfo,
                            vterm: *mut c_void)
                            -> c_int {
    0
}

/// Call the given closure with the vterms sender, if it exists.
fn with_sender<F>(vterm: *mut c_void, closure: F) -> c_int
    where F: Fn(&Sender<StateEvent>) -> c_int
{
    let vterm: &mut VTerm = unsafe { &mut *(vterm as *mut VTerm) };
    match vterm.state_event_tx.as_ref() {
        Some(tx) => closure(tx),
        None => 0,
    }
}
