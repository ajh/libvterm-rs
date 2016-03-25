use libc::{c_int, c_void};
use std::sync::mpsc::Sender;

use super::*;

// int (*putglyph)(VTermGlyphInfo *info, VTermPos pos, void *user);
extern "C" fn put_glyph(info: *mut ffi::VTermGlyphInfo,
                        pos: ffi::VTermPos,
                        vterm: *mut c_void)
                        -> c_int {
    with_sender(vterm, |tx| {
        let event = StateEvent::PutGlyph {
            glyph_info: ::GlyphInfo::from_ptr(info),
            pos: pos.as_pos(),
        };

        match tx.send(event) {
            Ok(_) => 1,
            Err(_) => 0,
        }
    })
}

// int (*movecursor)(VTermPos pos, VTermPos oldpos, int visible, void *user);
extern "C" fn move_cursor(new: ffi::VTermPos,
                          old: ffi::VTermPos,
                          visible: c_int,
                          vterm: *mut c_void)
                          -> c_int {
    with_sender(vterm, |tx| {
        let event = StateEvent::MoveCursor {
            new: new.as_pos(),
            old: old.as_pos(),
            is_visible: int_to_bool(visible),
        };

        match tx.send(event) {
            Ok(_) => 1,
            Err(_) => 0,
        }
    })
}

// int (*scrollrect)(VTermRect rect, int downward, int rightward, void *user);
extern "C" fn scroll_rect(rect: ffi::VTermRect,
                          downward: c_int,
                          rightward: c_int,
                          vterm: *mut c_void)
                          -> c_int {
    with_sender(vterm, |tx| {
        let event = StateEvent::ScrollRect {
            rect: rect.as_rect(),
            downward: downward as isize,
            rightward: rightward as isize,
        };

        match tx.send(event) {
            Ok(_) => 1,
            Err(_) => 0,
        }
    })
}

// int (*moverect)(VTermRect dest, VTermRect src, void *user);
extern "C" fn move_rect(dest: ffi::VTermRect, src: ffi::VTermRect, vterm: *mut c_void) -> c_int {
    0
}
// int (*erase)(VTermRect rect, int selective, void *user);
extern "C" fn erase(rect: ffi::VTermRect, selective: c_int, vterm: *mut c_void) -> c_int {
    0
}
// int (*initpen)(void *user);
extern "C" fn init_pen(vterm: *mut c_void) -> c_int {
    0
}
// int (*setpenattr)(VTermAttr attr, VTermValue *val, void *user);
extern "C" fn set_pen_attr(attr: ffi::VTermAttr,
                           val: *mut ffi::VTermValue,
                           vterm: *mut c_void)
                           -> c_int {
    0
}
// int (*settermprop)(VTermProp prop, VTermValue *val, void *user);
extern "C" fn set_term_prop(prop: ffi::VTermProp,
                            val: *mut ffi::VTermValue,
                            vterm: *mut c_void)
                            -> c_int {
    0
}
// int (*bell)(void *user);
extern "C" fn bell(vterm: *mut c_void) -> c_int {
    0
}
// int (*resize)(int rows, int cols, VTermPos *delta, void *user);
extern "C" fn resize(rows: c_int,
                     cols: c_int,
                     delta: *mut ffi::VTermPos,
                     vterm: *mut c_void)
                     -> c_int {
    0
}
// int (*setlineinfo)(int row, const VTermLineInfo *newinfo, const VTermLineInfo *oldinfo, void *user);
extern "C" fn set_line_info(row: c_int,
                            new: *const ffi::VTermLineInfo,
                            old: *const ffi::VTermLineInfo,
                            vterm: *mut c_void)
                            -> c_int {
    0
}

pub static STATE_CALLBACKS: ffi::VTermStateCallbacks = ffi::VTermStateCallbacks {
    put_glyph: put_glyph,
    move_cursor: move_cursor,
    scroll_rect: scroll_rect,
    move_rect: move_rect,
    erase: erase,
    init_pen: init_pen,
    set_pen_attr: set_pen_attr,
    set_term_prop: set_term_prop,
    bell: bell,
    resize: resize,
    set_line_info: set_line_info,
};

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
