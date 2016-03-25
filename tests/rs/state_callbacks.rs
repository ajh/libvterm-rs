use std::io::prelude::*;
use vterm_sys::*;
use term::terminfo::TermInfo;
use ::support::CapBuilder;
use std::sync::mpsc::Receiver;

#[test]
fn state_can_generate_put_glyph_events() {
    let mut vterm: VTerm = VTerm::new(&Size {
        height: 2,
        width: 2,
    });
    vterm.state_receive_events(&StateCallbacksConfig::all());
    vterm.write(b"a").unwrap();

    let rx = vterm.state_event_rx.take().unwrap();
    let event = try_recv_put_glyph_event(&rx);

    assert!(event.is_some());
    let event = event.unwrap();
    assert_eq!(event.glyph_info.chars[0], b'a');
    assert_eq!(event.pos.x, 0);
    assert_eq!(event.pos.y, 0);
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
    let event = try_recv_move_cursor_event(&rx);

    assert!(event.is_some());
    let event = event.unwrap();
    assert_eq!(event.new, Pos { x: 1, y: 0 });
    assert_eq!(event.old, Pos { x: 0, y: 0 });
    assert_eq!(event.is_visible, true);
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
    let event = try_recv_scroll_rect_event(&rx);

    assert!(event.is_some());
    let event = event.unwrap();
    assert_eq!(event.rect, Rect::new(Pos::new(0, 0), Size::new(2, 2)));
    assert_eq!(event.downward, -1);
    assert_eq!(event.rightward, 0);
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
    let event = try_recv_scroll_rect_event(&rx);

    assert!(event.is_some());
    let event = event.unwrap();
    assert_eq!(event.rect, Rect::new(Pos::new(0, 3), Size::new(2, 2)));
    assert_eq!(event.downward, -1);
    assert_eq!(event.rightward, 0);
}

fn try_recv_put_glyph_event(rx: &Receiver<StateEvent>) -> Option<PutGlyphEvent> {
    while let Ok(e) = rx.try_recv() {
        match e {
            StateEvent::PutGlyph(v) => return Some(v),
            _ => {}
        }
    }

    None
}

fn try_recv_move_cursor_event(rx: &Receiver<StateEvent>) -> Option<MoveCursorEvent> {
    while let Ok(e) = rx.try_recv() {
        match e {
            StateEvent::MoveCursor(v) => return Some(v),
            _ => {}
        }
    }

    None
}

fn try_recv_scroll_rect_event(rx: &Receiver<StateEvent>) -> Option<ScrollRectEvent> {
    while let Ok(e) = rx.try_recv() {
        match e {
            StateEvent::ScrollRect(v) => return Some(v),
            _ => {}
        }
    }

    None
}
