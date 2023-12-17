use crate::geometry::Point;
use std::cmp::{max, min};

#[derive(Copy, Clone, Debug)]
pub struct LineSegment<N> {
    pub start: Point<2, N>,
    pub end: Point<2, N>,
}

impl<N: Copy> From<(Point<2, N>, Point<2, N>)> for LineSegment<N> {
    fn from(value: (Point<2, N>, Point<2, N>)) -> Self {
        LineSegment { start: value.0, end: value.1 }
    }
}

impl<N: Copy + Ord> LineSegment<N> {
    pub fn min_x(&self) -> N { min(self.start.x(), self.end.x()) }
    pub fn min_y(&self) -> N { min(self.start.y(), self.end.y()) }
    pub fn max_x(&self) -> N { max(self.start.x(), self.end.x()) }
    pub fn max_y(&self) -> N { max(self.start.y(), self.end.y()) }
}
