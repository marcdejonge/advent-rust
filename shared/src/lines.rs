use rusttype::Point;
use std::cmp::{max, min};

#[derive(Copy, Clone, Debug)]
pub struct LineSegment<N> {
    pub start: Point<N>,
    pub end: Point<N>,
}

impl<N: Copy> Into<LineSegment<N>> for (Point<N>, Point<N>) {
    fn into(self) -> LineSegment<N> { LineSegment { start: self.0, end: self.1 } }
}

impl<N: Copy + Ord> LineSegment<N> {
    pub fn min_x(&self) -> N { min(self.start.x, self.end.x) }
    pub fn min_y(&self) -> N { min(self.start.y, self.end.y) }
    pub fn max_x(&self) -> N { max(self.start.x, self.end.x) }
    pub fn max_y(&self) -> N { max(self.start.y, self.end.y) }
}
