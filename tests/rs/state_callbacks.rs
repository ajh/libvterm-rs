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

//fn state_can_generate_move_rect_events()
//fn state_can_generate_erase_events()
//fn state_can_generate_init_pen_events()
//fn state_can_generate_set_pen_attr_events()

#[test]
fn state_can_generate_cursor_visible_events() {
    let mut vterm: VTerm = VTerm::new(&Size {
        height: 2,
        width: 2,
    });
    vterm.state_receive_events(&StateCallbacksConfig::all());

    let terminfo = TermInfo::from_name("xterm").unwrap();
    vterm.write(&CapBuilder::new(&terminfo)
                     .cap("civis")
                     .build()
                     .unwrap()).unwrap();
    vterm.flush().unwrap();

    let rx = vterm.state_event_rx.take().unwrap();
    let event: Option<CursorVisibleEvent> = try_recv_cursor_visible_event(&rx);

    assert!(event.is_some());
    let event = event.unwrap();
    assert_eq!(event.is_true, false);

    vterm.write(&CapBuilder::new(&terminfo)
                     .cap("cnorm")
                     .build()
                     .unwrap()).unwrap();
    vterm.flush().unwrap();

    let event: Option<CursorVisibleEvent> = try_recv_cursor_visible_event(&rx);

    assert!(event.is_some());
    let event = event.unwrap();
    assert_eq!(event.is_true, true);
}

#[test]
fn state_can_generate_cursor_blink_events() {
    let mut vterm: VTerm = VTerm::new(&Size {
        height: 2,
        width: 2,
    });
    vterm.state_receive_events(&StateCallbacksConfig::all());

    // DECSCUSR
    vterm.write(b"\x1b[1 q").unwrap();
    vterm.flush().unwrap();

    let rx = vterm.state_event_rx.take().unwrap();
    let event: Option<CursorBlinkEvent> = try_recv_cursor_blink_event(&rx);

    assert!(event.is_some());
    let event = event.unwrap();
    assert_eq!(event.is_true, true);

    vterm.write(b"\x1b[2 q").unwrap();
    vterm.flush().unwrap();

    let event: Option<CursorBlinkEvent> = try_recv_cursor_blink_event(&rx);

    assert!(event.is_some());
    let event = event.unwrap();
    assert_eq!(event.is_true, false);
}

#[test]
fn state_can_generate_cursor_shape_events() {
    let mut vterm: VTerm = VTerm::new(&Size {
        height: 2,
        width: 2,
    });
    vterm.state_receive_events(&StateCallbacksConfig::all());

    // DECSCUSR sequence
    vterm.write(b"\x1b[4 q").unwrap();
    vterm.flush().unwrap();

    let rx = vterm.state_event_rx.take().unwrap();
    let event: Option<CursorShapeEvent> = try_recv_cursor_shape_event(&rx);

    assert!(event.is_some());
    let event = event.unwrap();
    // TODO: this should be an enum value, not an integer
    assert_eq!(event.value, 2);

    vterm.write(b"\x1b[0 q").unwrap();
    vterm.flush().unwrap();

    let event: Option<CursorShapeEvent> = try_recv_cursor_shape_event(&rx);

    assert!(event.is_some());
    let event = event.unwrap();
    assert_eq!(event.value, 1);
}

#[test]
fn state_can_generate_title_events() {
    let mut vterm: VTerm = VTerm::new(&Size {
        height: 2,
        width: 2,
    });
    vterm.state_receive_events(&StateCallbacksConfig::all());

    // DECSWT
    vterm.write(b"\x1b]2;foo\x1b\\").unwrap();
    vterm.flush().unwrap();

    let rx = vterm.state_event_rx.take().unwrap();
    let event: Option<TitleEvent> = try_recv_title_event(&rx);

    assert!(event.is_some());
    let event = event.unwrap();
    assert_eq!(event.text, "foo");

    vterm.write(b"\x1b]2;bar\x1b\\").unwrap();
    vterm.flush().unwrap();

    let event: Option<TitleEvent> = try_recv_title_event(&rx);

    assert!(event.is_some());
    let event = event.unwrap();
    assert_eq!(event.text, "bar");
}

#[test]
fn state_can_generate_iconname_events() {
    let mut vterm: VTerm = VTerm::new(&Size {
        height: 2,
        width: 2,
    });
    vterm.state_receive_events(&StateCallbacksConfig::all());

    // DECSWT
    vterm.write(b"\x1b]1;foo\x1b\\").unwrap();
    vterm.flush().unwrap();

    let rx = vterm.state_event_rx.take().unwrap();
    let event: Option<IconNameEvent> = try_recv_icon_name_event(&rx);

    assert!(event.is_some());
    let event = event.unwrap();
    assert_eq!(event.text, "foo");

    vterm.write(b"\x1b]1;bar\x1b\\").unwrap();
    vterm.flush().unwrap();

    let event: Option<IconNameEvent> = try_recv_icon_name_event(&rx);

    assert!(event.is_some());
    let event = event.unwrap();
    assert_eq!(event.text, "bar");
}

#[test]
fn state_can_generate_reverse_events() {
    let mut vterm: VTerm = VTerm::new(&Size {
        height: 2,
        width: 2,
    });
    vterm.state_receive_events(&StateCallbacksConfig::all());

    // DECSCNM
    vterm.write(b"\x1b[?5h").unwrap();
    vterm.flush().unwrap();

    let rx = vterm.state_event_rx.take().unwrap();
    let event: Option<ReverseEvent> = try_recv_reverse_event(&rx);

    assert!(event.is_some());
    let event = event.unwrap();
    assert_eq!(event.is_true, true);

    vterm.write(b"\x1b[?5l").unwrap();
    vterm.flush().unwrap();

    let event: Option<ReverseEvent> = try_recv_reverse_event(&rx);

    assert!(event.is_some());
    let event = event.unwrap();
    assert_eq!(event.is_true, false);
}

#[test]
fn state_can_generate_mouse_events() {
    let mut vterm: VTerm = VTerm::new(&Size {
        height: 2,
        width: 2,
    });
    vterm.state_receive_events(&StateCallbacksConfig::all());

    // DECSET x10 mouse support
    vterm.write(b"\x1b[?1003h").unwrap();
    vterm.flush().unwrap();

    let rx = vterm.state_event_rx.take().unwrap();
    let event: Option<MouseEvent> = try_recv_mouse_event(&rx);

    assert!(event.is_some());
    let event = event.unwrap();

    // TODO: this should be an enum value, not an integer
    assert_eq!(event.value, 3);
}

#[test]
fn state_can_generate_alt_screen_events() {
    let mut vterm: VTerm = VTerm::new(&Size {
        height: 2,
        width: 2,
    });
    vterm.state_receive_events(&StateCallbacksConfig::all());

    let terminfo = TermInfo::from_name("xterm").unwrap();
    vterm.write(&CapBuilder::new(&terminfo)
                     .cap("smcup")
                     .build()
                     .unwrap()).unwrap();
    vterm.flush().unwrap();

    let rx = vterm.state_event_rx.take().unwrap();
    let event: Option<AltScreenEvent> = try_recv_alt_screen_event(&rx);

    assert!(event.is_some());
    let event = event.unwrap();
    assert_eq!(event.is_true, true);

    vterm.write(&CapBuilder::new(&terminfo)
                     .cap("rmcup")
                     .build()
                     .unwrap()).unwrap();
    vterm.flush().unwrap();

    let event: Option<AltScreenEvent> = try_recv_alt_screen_event(&rx);

    assert!(event.is_some());
    let event = event.unwrap();
    assert_eq!(event.is_true, false);
}

// TODO: Figure out some way to DRY this up please!

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

fn try_recv_alt_screen_event(rx: &Receiver<StateEvent>) -> Option<AltScreenEvent> {
    while let Ok(e) = rx.try_recv() {
        match e {
            StateEvent::AltScreen(v) => return Some(v),
            _ => {}
        }
    }

    None
}

fn try_recv_cursor_visible_event(rx: &Receiver<StateEvent>) -> Option<CursorVisibleEvent> {
    while let Ok(e) = rx.try_recv() {
        match e {
            StateEvent::CursorVisible(v) => return Some(v),
            _ => {}
        }
    }

    None
}

fn try_recv_cursor_blink_event(rx: &Receiver<StateEvent>) -> Option<CursorBlinkEvent> {
    while let Ok(e) = rx.try_recv() {
        match e {
            StateEvent::CursorBlink(v) => return Some(v),
            _ => {}
        }
    }

    None
}

fn try_recv_cursor_shape_event(rx: &Receiver<StateEvent>) -> Option<CursorShapeEvent> {
    while let Ok(e) = rx.try_recv() {
        match e {
            StateEvent::CursorShape(v) => return Some(v),
            _ => {}
        }
    }

    None
}

fn try_recv_title_event(rx: &Receiver<StateEvent>) -> Option<TitleEvent> {
    while let Ok(e) = rx.try_recv() {
        match e {
            StateEvent::Title(v) => return Some(v),
            _ => {}
        }
    }

    None
}

fn try_recv_icon_name_event(rx: &Receiver<StateEvent>) -> Option<IconNameEvent> {
    while let Ok(e) = rx.try_recv() {
        match e {
            StateEvent::IconName(v) => return Some(v),
            _ => {}
        }
    }

    None
}

fn try_recv_reverse_event(rx: &Receiver<StateEvent>) -> Option<ReverseEvent> {
    while let Ok(e) = rx.try_recv() {
        match e {
            StateEvent::Reverse(v) => return Some(v),
            _ => {}
        }
    }

    None
}

fn try_recv_mouse_event(rx: &Receiver<StateEvent>) -> Option<MouseEvent> {
    while let Ok(e) = rx.try_recv() {
        match e {
            StateEvent::Mouse(v) => return Some(v),
            _ => {}
        }
    }

    None
}
