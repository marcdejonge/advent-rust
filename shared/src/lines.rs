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
    pub fn min_x(&self) -> N { min(self.start.coords[0], self.end.coords[0]) }
    pub fn min_y(&self) -> N { min(self.start.coords[1], self.end.coords[1]) }
    pub fn max_x(&self) -> N { max(self.start.coords[0], self.end.coords[0]) }
    pub fn max_y(&self) -> N { max(self.start.coords[1], self.end.coords[1]) }
}
