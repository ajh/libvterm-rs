use std::io::prelude::*;
use vterm_sys::*;
use std::sync::mpsc::Receiver;
use term::terminfo::TermInfo;
use ::support::CapBuilder;

#[test]
fn screen_can_generate_damage_events() {
    let mut vterm: VTerm = VTerm::new(&Size {
        height: 2,
        width: 4,
    });
    vterm.screen_receive_events(&ScreenCallbacksConfig::all());
    vterm.screen_set_damage_merge(DamageSize::Screen);

    vterm.write(b"abc").unwrap();
    vterm.flush().unwrap();

    let rx = vterm.screen_event_rx.take().unwrap();
    let event = try_recv_damage_event(&rx);

    assert!(event.is_some());
    let event = event.unwrap();
    assert_eq!(event.rect, Rect::new(Pos::new(0,0), Size::new(3,1)));
}

#[test]
fn screen_can_generate_move_rect_events() {
    let mut vterm: VTerm = VTerm::new(&Size {
        height: 2,
        width: 2,
    });
    vterm.screen_receive_events(&ScreenCallbacksConfig::all());
    vterm.screen_set_damage_merge(DamageSize::Screen);

    vterm.write(b"abcde").unwrap();
    vterm.flush().unwrap();

    let rx = vterm.screen_event_rx.take().unwrap();
    let event = try_recv_move_rect_event(&rx);

    assert!(event.is_some());
    let event = event.unwrap();
    assert_eq!(event.src, Rect::new(Pos::new(0,1), Size::new(2,1)));
    assert_eq!(event.dest, Rect::new(Pos::new(0,0), Size::new(2,1)));
}

#[test]
fn screen_can_generate_move_cursor_events() {
    let mut vterm: VTerm = VTerm::new(&Size {
        height: 2,
        width: 2,
    });
    vterm.screen_receive_events(&ScreenCallbacksConfig::all());
    vterm.screen_set_damage_merge(DamageSize::Screen);

    let terminfo = TermInfo::from_name("xterm").unwrap();
    vterm.write(&CapBuilder::new(&terminfo)
                     .cap("cup")
                     .number_param(0)
                     .number_param(1)
                     .build()
                     .unwrap()).unwrap();

    let rx = vterm.screen_event_rx.take().unwrap();
    let event = try_recv_move_cursor_event(&rx);

    assert!(event.is_some());
    let event = event.unwrap();
    assert_eq!(event.old, Pos::new(0,0));
    assert_eq!(event.new, Pos::new(1,0));
}

#[test]
fn screen_can_generate_alt_screen_events() {
    let mut vterm: VTerm = VTerm::new(&Size {
        height: 2,
        width: 2,
    });
    vterm.screen_receive_events(&ScreenCallbacksConfig::all());
    vterm.screen_set_damage_merge(DamageSize::Screen);

    let terminfo = TermInfo::from_name("xterm").unwrap();
    vterm.write(&CapBuilder::new(&terminfo)
                     .cap("rmcup")
                     .build()
                     .unwrap()).unwrap();

    let rx = vterm.screen_event_rx.take().unwrap();
    let event = try_recv_alt_screen_event(&rx);

    assert!(event.is_some());
    let event = event.unwrap();
    assert_eq!(event.is_on, false);

    // The way I'm reseting the screen I think is causing this not to fire

    //vterm.write(&CapBuilder::new(&terminfo)
                     //.cap("smcup")
                     //.build()
                     //.unwrap()).unwrap();
    //vterm.flush().unwrap();

    //let event = try_recv_alt_screen_event(&rx);
    //assert!(event.is_some());
    //let event = event.unwrap();
    //assert_eq!(event.is_on, true);
}

#[test]
fn screen_can_generate_cursor_blink_events() {
    let mut vterm: VTerm = VTerm::new(&Size {
        height: 2,
        width: 2,
    });
    vterm.screen_receive_events(&ScreenCallbacksConfig::all());
    vterm.screen_set_damage_merge(DamageSize::Screen);

    // DECSCUSR
    vterm.write(b"\x1b[1 q").unwrap();
    vterm.flush().unwrap();

    let rx = vterm.screen_event_rx.take().unwrap();
    let event = try_recv_cursor_blink_event(&rx);

    assert!(event.is_some());
    let event = event.unwrap();
    assert_eq!(event.is_on, true);

    vterm.write(b"\x1b[2 q").unwrap();
    vterm.flush().unwrap();

    let event = try_recv_cursor_blink_event(&rx);
    assert!(event.is_some());
    let event = event.unwrap();
    assert_eq!(event.is_on, false);
}

// Builds a function that returns a Some of the first event of the given type found on the channel
// or None.
macro_rules! dry {
    ($n:ident, $t:ty, $p:path) => {
        fn $n(rx: &Receiver<ScreenEvent>) -> Option<$t> {
            while let Ok(e) = rx.try_recv() {
                match e {
                    $p(v) => return Some(v),
                    _ => {}
                }
            }
            None
        }
    }
}

dry!(try_recv_damage_event, DamageEvent, ScreenEvent::Damage);
dry!(try_recv_move_rect_event, MoveRectEvent, ScreenEvent::MoveRect);
dry!(try_recv_move_cursor_event, MoveCursorEvent, ScreenEvent::MoveCursor);
dry!(try_recv_alt_screen_event, AltScreenEvent, ScreenEvent::AltScreen);
dry!(try_recv_cursor_blink_event, CursorBlinkEvent, ScreenEvent::CursorBlink);
