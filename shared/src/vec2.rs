use std::fmt::{Display, Formatter};
use std::ops::{Add, RangeInclusive};
use std::str::FromStr;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Vec2<T> {
    pub x: T,
    pub y: T,
}

impl<T: FromStr> FromStr for Vec2<T> {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let ix = s.find(",").ok_or(())?;
        let x = T::from_str(&s[0..ix]).map_err(|_| ())?;
        let y = T::from_str(&s[(ix + 1)..]).map_err(|_| ())?;
        Ok(Vec2 { x, y })
    }
}

impl<T: Display> Display for Vec2<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.x.fmt(f)?;
        f.write_str(",")?;
        self.y.fmt(f)?;
        Ok(())
    }
}

impl<T: Add<Output = T>> Add for Vec2<T> {
    type Output = Vec2<T>;

    fn add(self, rhs: Self) -> Self::Output {
        Vec2 { x: self.x + rhs.x, y: self.y + rhs.y }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct LineSegment<T> {
    pub start: Vec2<T>,
    pub end: Vec2<T>,
}

impl<T: Copy> LineSegment<T> {
    pub fn x_range(&self) -> RangeInclusive<T> {
        self.start.x..=self.end.x
    }

    pub fn y_range(&self) -> RangeInclusive<T> {
        self.start.y..=self.end.y
    }
}

impl<T: Copy + Ord + PartialEq> Into<LineSegment<T>> for (Vec2<T>, Vec2<T>) {
    fn into(self) -> LineSegment<T> {
        let start = if self.0.x < self.1.x || self.0.y < self.1.y { self.0 } else { self.1 };
        let end = if start == self.1 { self.0 } else { self.1 };
        LineSegment { start, end }
    }
}

impl<T: Display> Display for LineSegment<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.start.fmt(f)?;
        f.write_str(" -> ")?;
        self.end.fmt(f)?;
        Ok(())
    }
}
