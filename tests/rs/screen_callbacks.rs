use std::io::prelude::*;
use vterm_sys::*;
use term::terminfo::TermInfo;
use ::support::CapBuilder;
use std::sync::mpsc::Receiver;

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
    let event: Option<DamageEvent> = try_recv_damage_event(&rx);

    assert!(event.is_some());
    let event = event.unwrap();
    assert_eq!(event.rect, Rect::new(Pos::new(0,0), Size::new(3,1)));
}

fn try_recv_damage_event(rx: &Receiver<ScreenEvent>) -> Option<DamageEvent> {
    while let Ok(e) = rx.try_recv() {
        match e {
            ScreenEvent::Damage(v) => return Some(v),
            _ => {}
        }
    }

    None
}
