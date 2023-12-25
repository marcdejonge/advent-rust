#![feature(test)]

use std::fmt::{Debug, Formatter};

use prse::*;

use advent_lib::day::*;
use advent_lib::geometry::{point2, BoundingBox, Point, Vector};

struct Day {
    hail: Vec<Line<3>>,
}

#[derive(Parse)]
#[prse = "{p} @ {v}"]
struct Line<const D: usize> {
    p: Point<D, i128>,
    v: Vector<D, i128>,
}

impl<const D: usize> Debug for Line<D> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.p.fmt(f)?;
        f.write_str(" @ ")?;
        self.v.fmt(f)?;

        Ok(())
    }
}

impl Line<3> {
    fn flatten(&self) -> Line<2> {
        Line { p: self.p.switch_dimensions(), v: self.v.switch_dimensions() }
    }
}

impl Line<2> {
    // Based on answer here https://stackoverflow.com/questions/563198/how-do-you-detect-where-two-line-segments-intersect
    // Adjusted to no longer require a division, so it's only integer calculations.
    // p = self.p, q = other.p, r = self.v, s = other.v
    fn intersects_in(&self, other: &Line<2>, min: Point<2, i128>, max: Point<2, i128>) -> bool {
        let cross = self.v.cross(other.v);
        if cross == 0 {
            return false; // Parallel lines
        }

        let diff = other.p - self.p;

        let t_times_cross = diff.cross(other.v);
        if cross * t_times_cross < 0 {
            return false; // Intersection is in the past for self
        }

        let u_times_cross = diff.cross(self.v);
        if cross * u_times_cross < 0 {
            return false; // Intersection is in the past for other
        }

        let intersect_times_cross = self.p * cross + self.v * t_times_cross;
        BoundingBox::from(min * cross, max * cross).contains_inclusive(&intersect_times_cross)
    }
}

impl ExecutableDay for Day {
    type Output = usize;

    fn from_lines<LINES: Iterator<Item = String>>(lines: LINES) -> Self {
        Day { hail: lines.map(|line| parse!(line, "{}")).collect() }
    }

    fn calculate_part1(&self) -> Self::Output {
        let hail2d = self.hail.iter().map(Line::flatten).collect::<Vec<_>>();
        let (min, max) = if self.hail.len() < 10 {
            (point2(7, 7), point2(27, 27))
        } else {
            (
                point2(200000000000000, 200000000000000),
                point2(400000000000000, 400000000000000),
            )
        };

        let mut count = 0;
        for ix in 0..hail2d.len() {
            for jx in (ix + 1)..hail2d.len() {
                if hail2d[ix].intersects_in(&hail2d[jx], min, max) {
                    count += 1;
                }
            }
        }

        count
    }

    fn calculate_part2(&self) -> Self::Output { todo!() }
}

fn main() { execute_day::<Day>() }

#[cfg(test)]
mod tests {
    use advent_lib::day_test;

    day_test!( 24, example => 2 );
    day_test!( 24 => 19523 );
}
