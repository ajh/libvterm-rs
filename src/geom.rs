use euclid;

pub type Size = euclid::Size2D<usize>;
pub type Pos = euclid::Point2D<usize>;
pub type Rect = euclid::Rect<usize>;

pub trait RectAssist {
    fn top(&self) -> usize;
    fn left(&self) -> usize;
    fn bottom(&self) -> usize;
    fn right(&self) -> usize;

    /// iterator of positions in by row order, meaning positions for all columns are returned for
    /// the first row, before positions for subsequent rows.
    fn positions<'a>(&'a self) -> RectPositions<'a>;
}

impl RectAssist for Rect {
    /// TODO: remove these because I think these already exist as min_x, max_x etc
    fn top(&self) -> usize {
        self.origin.y
    }
    fn left(&self) -> usize {
        self.origin.x
    }
    fn bottom(&self) -> usize {
        self.origin.y + self.size.height
    }
    fn right(&self) -> usize {
        self.origin.x + self.size.width
    }

    fn positions<'a>(&'a self) -> RectPositions<'a> {
        RectPositions::new(self)
    }
}

#[derive(Debug)]
pub struct RectPositions<'a> {
    pub rect: &'a Rect,
    pos: Pos,
}

impl<'a> RectPositions<'a> {
    pub fn new(rect: &'a Rect) -> RectPositions<'a> {
        RectPositions {
            rect: rect,
            pos: rect.origin.clone(),
        }
    }

    fn advance(&mut self) {
        if self.pos.x + 1 < self.rect.right() {
            self.pos.x += 1;
        }
        else {
            self.pos.y += 1;
            self.pos.x = self.rect.origin.x;
        }
    }
}

impl<'a> Iterator for RectPositions<'a> {
    type Item = Pos;
    // The 'Iterator' trait only requires the 'next' method to be defined. The
    // return type is 'Option<T>', 'None' is returned when the 'Iterator' is
    // over, otherwise the next value is returned wrapped in 'Some'
    fn next(&mut self) -> Option<Pos> {
        if self.rect.contains(&self.pos) {
            let output = self.pos.clone();
            self.advance();
            Some(output)
        }
        else {
            None
        }
    }
}

mod tests {
    #![allow(unused_imports)]
    use super::*;

    #[test]
    fn rect_positions_will_iterate() {
        let rect = Rect::new(Pos { x: 0, y: 0 }, Size { width: 2, height: 3 });
        let mut iter = rect.positions();
        assert_eq!(iter.next(), Some(Pos { x: 0, y: 0 }));
        assert_eq!(iter.next(), Some(Pos { x: 1, y: 0 }));
        assert_eq!(iter.next(), Some(Pos { x: 0, y: 1 }));
        assert_eq!(iter.next(), Some(Pos { x: 1, y: 1 }));
        assert_eq!(iter.next(), Some(Pos { x: 0, y: 2 }));
        assert_eq!(iter.next(), Some(Pos { x: 1, y: 2 }));
    }
}
