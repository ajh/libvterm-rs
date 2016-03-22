use libc::{c_int, c_void};
use std::sync::mpsc::Sender;

use super::*;

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
    0
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

mod tests {
    #![allow(unused_imports)]
    use super::super::*;
    use std::io::prelude::*;

    #[test]
    fn state_can_generate_put_glyph_events() {
        let mut vterm: VTerm = VTerm::new(&Size {
            height: 2,
            width: 2,
        });
        vterm.generate_state_events().unwrap();
        vterm.write(b"a");

        let rx = vterm.state_event_rx.take().unwrap();

        let mut found_it = false;
        while let Ok(e) = rx.try_recv() {
            match e {
                StateEvent::PutGlyph{glyph_info, pos} => {
                    found_it = true;

                    assert_eq!(glyph_info.chars[0], b'a');
                    assert_eq!(pos.x, 0);
                    assert_eq!(pos.y, 0);

                    break;
                }
                _ => {}
            }
        }

        assert!(found_it);
    }

    #[test]
    fn state_can_generate_move_cursor_events() {
        let mut vterm: VTerm = VTerm::new(&Size {
            height: 2,
            width: 2,
        });
        vterm.generate_state_events().unwrap();
        vterm.write(b"\x1b[1;2H");

        let rx = vterm.state_event_rx.take().unwrap();

        let mut found_it = false;
        while let Ok(e) = rx.try_recv() {
            match e {
                StateEvent::MoveCursor{new, old, is_visible} => {
                    found_it = true;

                    assert_eq!(new, Pos { x: 1, y: 0 });
                    assert_eq!(old, Pos { x: 0, y: 0 });
                    assert_eq!(is_visible, true);

                    break;
                }
                _ => {}
            }
        }

        assert!(found_it);
    }
}
