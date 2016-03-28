use {ScreenCell, GlyphInfo, Rect, Pos, ColorRGB, ColorPalette};

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
    pub shape: CursorShape,
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
    pub mode: MouseMode,
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
    /// true when DECSED used, false if ED used
    pub is_selective: bool,
}
#[derive(PartialEq, Debug, Clone)]
pub struct InitPenEvent;
#[derive(PartialEq, Debug, Clone)]
pub struct BellEvent;
#[derive(PartialEq, Debug, Clone)]
pub struct PenBoldEvent {
    pub is_true: bool
}
#[derive(PartialEq, Debug, Clone)]
pub struct PenUnderlineEvent {
    pub value: Underline
}
#[derive(PartialEq, Debug, Clone)]
pub struct PenItalicEvent {
    pub is_true: bool
}
#[derive(PartialEq, Debug, Clone)]
pub struct PenBlinkEvent {
    pub is_true: bool
}
#[derive(PartialEq, Debug, Clone)]
pub struct PenReverseEvent {
    pub is_true: bool
}
#[derive(PartialEq, Debug, Clone)]
pub struct PenStrikeEvent {
    pub is_true: bool
}
#[derive(PartialEq, Debug, Clone)]
pub struct PenFontEvent {
    pub value: i32
}
#[derive(PartialEq, Debug, Clone)]
pub struct PenForegroundEvent {
    pub rgb: ColorRGB,
    pub palette: ColorPalette
}
#[derive(PartialEq, Debug, Clone)]
pub struct PenBackgroundEvent {
    pub rgb: ColorRGB,
    pub palette: ColorPalette
}
#[derive(PartialEq, Debug, Clone)]
pub enum CursorShape {
  Block = 1,
  Underline,
  BarLeft,
}

impl CursorShape {
    pub fn from_i32(val: i32) -> CursorShape {
        match val {
            1 => CursorShape::Block,
            2 => CursorShape::Underline,
            3 => CursorShape::BarLeft,
            _ => panic!("unknown cursor shape value: {}", val)
        }
    }
}

#[derive(PartialEq, Debug, Clone)]
pub enum MouseMode {
  None = 0,
  Click,
  Drag,
  Move,
}

impl MouseMode {
    pub fn from_i32(val: i32) -> MouseMode {
        match val {
            0 => MouseMode::None,
            1 => MouseMode::Click,
            2 => MouseMode::Drag,
            3 => MouseMode::Move,
            _ => panic!("unknown mouse mode value: {}", val)
        }
    }
}

#[derive(PartialEq, Debug, Clone)]
pub enum Underline {
  None = 0,
  Single,
  Double,
}

impl Underline {
    pub fn from_i32(val: i32) -> Underline {
        match val {
            0 => Underline::None,
            1 => Underline::Single,
            2 => Underline::Double,
            _ => panic!("unknown underline value: {}", val)
        }
    }
}
