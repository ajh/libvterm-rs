use std::io::prelude::*;
use vterm_sys::*;
use term::terminfo::TermInfo;
use ::support::CapBuilder;

#[test]
fn state_can_generate_put_glyph_events() {
    let mut vterm: VTerm = VTerm::new(&Size {
        height: 2,
        width: 2,
    });
    vterm.state_receive_events(&StateCallbacksConfig::all());
    vterm.write(b"a").unwrap();

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
    vterm.state_receive_events(&StateCallbacksConfig::all());

    let terminfo = TermInfo::from_name("xterm").unwrap();
    vterm.write(&CapBuilder::new(&terminfo)
                     .cap("cup")
                     .number_param(0)
                     .number_param(1)
                     .build()
                     .unwrap()).unwrap();

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

#[test]
fn state_can_generate_scroll_rect_events() {
    let mut vterm: VTerm = VTerm::new(&Size {
        height: 2,
        width: 2,
    });
    vterm.state_receive_events(&StateCallbacksConfig::all());
    let terminfo = TermInfo::from_name("xterm").unwrap();
    vterm.write(&CapBuilder::new(&terminfo)
                     .cap("ri")
                     .build()
                     .unwrap()).unwrap();

    let rx = vterm.state_event_rx.take().unwrap();

    let mut found_it = false;
    while let Ok(e) = rx.try_recv() {
        match e {
            StateEvent::ScrollRect{rect, downward, rightward} => {
                found_it = true;

                assert_eq!(rect, Rect::new(Pos::new(0, 0), Size::new(2, 2)));
                assert_eq!(downward, -1);
                assert_eq!(rightward, 0);

                break;
            }
            _ => {}
        }
    }

    assert!(found_it);
}

#[test]
fn state_can_generate_scroll_rect_events_for_part_of_screen() {
    let mut vterm: VTerm = VTerm::new(&Size {
        height: 5,
        width: 2,
    });
    vterm.state_receive_events(&StateCallbacksConfig::all());
    let terminfo = TermInfo::from_name("xterm").unwrap();

    vterm.write(&CapBuilder::new(&terminfo)
                     .cap("csr")
                     .number_param(3)
                     .number_param(5)
                     .build()
                     .unwrap()).unwrap();

    vterm.write(&CapBuilder::new(&terminfo)
                     .cap("cup")
                     .number_param(3)
                     .number_param(0)
                     .build()
                     .unwrap()).unwrap();

    vterm.write(&CapBuilder::new(&terminfo)
                     .cap("ri")
                     .build()
                     .unwrap()).unwrap();

    let rx = vterm.state_event_rx.take().unwrap();

    let mut found_it = false;
    while let Ok(e) = rx.try_recv() {
        match e {
            StateEvent::ScrollRect{rect, downward, rightward} => {
                found_it = true;

                assert_eq!(rect, Rect::new(Pos::new(0, 3), Size::new(2, 2)));
                assert_eq!(downward, -1);
                assert_eq!(rightward, 0);

                break;
            }
            _ => {}
        }
    }

    assert!(found_it);
}
