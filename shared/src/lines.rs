use crate::direction::Direction;
use crate::geometry::{Point, Vector};
use nom_parse_macros::parse_from;
use std::cmp::{max, min};
use std::ops::Add;

#[derive(Copy, Clone)]
#[parse_from(separated_pair({}, (space0, "->", space0), {}) where N: Default + Copy)]
pub struct LineSegment<const D: usize, N> {
    pub start: Point<D, N>,
    pub end: Point<D, N>,
}

impl<const D: usize, N: std::fmt::Debug> std::fmt::Debug for LineSegment<D, N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Line({:?}, {:?})", self.start, self.end)
    }
}

impl<const D: usize, N: Copy> From<(Point<D, N>, Point<D, N>)> for LineSegment<D, N> {
    fn from(value: (Point<D, N>, Point<D, N>)) -> Self {
        LineSegment { start: value.0, end: value.1 }
    }
}

impl<N: Copy + Ord> LineSegment<2, N> {
    pub fn min_x(&self) -> N { min(self.start.x(), self.end.x()) }
    pub fn min_y(&self) -> N { min(self.start.y(), self.end.y()) }
    pub fn max_x(&self) -> N { max(self.start.x(), self.end.x()) }
    pub fn max_y(&self) -> N { max(self.start.y(), self.end.y()) }
}

impl<N: PartialEq + PartialOrd> LineSegment<2, N> {
    pub fn is_vertical(&self) -> bool { self.start.coords[0] == self.end.coords[0] }
    pub fn is_horizontal(&self) -> bool { self.start.coords[1] == self.end.coords[1] }
    pub fn get_direction(&self) -> Option<Direction> {
        if self.is_vertical() {
            if self.start.coords[1] < self.end.coords[1] {
                Some(Direction::South)
            } else {
                Some(Direction::North)
            }
        } else if self.is_horizontal() {
            if self.start.coords[0] < self.end.coords[0] {
                Some(Direction::East)
            } else {
                Some(Direction::West)
            }
        } else {
            None
        }
    }
}

impl<const D: usize, N: Add<Output = N> + Copy> Add<Vector<D, N>> for LineSegment<D, N> {
    type Output = LineSegment<D, N>;

    fn add(self, rhs: Vector<D, N>) -> Self::Output {
        LineSegment { start: self.start + rhs, end: self.end + rhs }
    }
}
