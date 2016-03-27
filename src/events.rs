use {ScreenCell, GlyphInfo, Rect, Pos};

#[derive(PartialEq, Debug, Clone)]
pub struct AltScreenEvent {
    /// this is a dumb name. Maybe `is_on` or just `value`?
    pub is_true: bool,
}
#[derive(PartialEq, Debug, Clone)]
pub struct CursorBlinkEvent {
    pub is_true: bool,
}
#[derive(PartialEq, Debug, Clone)]
pub struct CursorShapeEvent {
    pub value: i32,
}
#[derive(PartialEq, Debug, Clone)]
pub struct CursorVisibleEvent {
    pub is_true: bool,
}
#[derive(PartialEq, Debug, Clone)]
pub struct DamageEvent {
    pub rect: Rect,
}
#[derive(PartialEq, Debug, Clone)]
pub struct IconNameEvent {
    pub text: String,
}
#[derive(PartialEq, Debug, Clone)]
pub struct MouseEvent {
    pub value: i32,
}
#[derive(PartialEq, Debug, Clone)]
pub struct MoveCursorEvent {
    pub new: Pos,
    pub old: Pos,
    pub is_visible: bool,
}
#[derive(PartialEq, Debug, Clone)]
pub struct MoveRectEvent {
    pub dest: Rect,
    pub src: Rect,
}
#[derive(PartialEq, Debug, Clone)]
pub struct ResizeEvent {
    pub height: usize,
    pub width: usize,
}
#[derive(PartialEq, Debug, Clone)]
pub struct ReverseEvent {
    pub is_true: bool,
}
#[derive(PartialEq, Debug, Clone)]
pub struct SbPopLineEvent {
    pub cells: Vec<ScreenCell>,
}
#[derive(PartialEq, Debug, Clone)]
pub struct SbPushLineEvent {
    pub cells: Vec<ScreenCell>,
}
#[derive(PartialEq, Debug, Clone)]
pub struct TitleEvent {
    pub text: String,
}
#[derive(PartialEq, Debug, Clone)]
pub struct PutGlyphEvent {
    pub glyph_info: GlyphInfo,
    pub pos: Pos,
}
#[derive(PartialEq, Debug, Clone)]
pub struct ScrollRectEvent {
    pub rect: Rect,
    pub downward: isize,
    pub rightward: isize,
}
#[derive(PartialEq, Debug, Clone)]
pub struct EraseEvent {
    pub rect: Rect,
    /// ?
    pub selective: isize,
}
