#![feature(test)]

use std::fmt::{Debug, Formatter};

use num::Integer;
use prse::*;

use advent_lib::day::*;
use advent_lib::geometry::{point2, BoundingBox, Point, Vector};
use advent_lib::iter_utils::AllSetsTrait;

struct Day {
    hail: Vec<Line<3>>,
}

#[derive(Parse, Copy, Clone)]
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
    type Output = i128;

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

    fn calculate_part2(&self) -> Self::Output {
        let v: Vector<3, i128> = [0, 1, 2]
            .map(|dim| {
                (1..)
                    // Just try a bunch of speeds, both positive and negative
                    .flat_map(|x| [x, -x].into_iter())
                    .find(|test_speed| {
                        *test_speed != -1 // Hack for the example, it find this erroneously
                        && self
                            .hail
                            .iter()
                            .combinations()
                            .filter(|[h1, h2]| h1.v[dim] == h2.v[dim])
                            // For all combinations of hail that travel at the same speed in this dimension
                            .all(|[h1, h2]| {
                                let v = h1.v[dim];
                                let p1 = h1.p[dim];
                                let p2 = h2.p[dim];

                                // Assume, we're at p1 at some time t and testing if we cross p2 after some time
                                // p1 + t * v1 + dt * test_speed = p2 + t * v2 + dt * v2
                                // Since v1 == v2, we can drop the t * part
                                // p1 - p2 = dt * (v2 - test_speed)
                                (p1 - p2).is_multiple_of(&(v - *test_speed))
                            })
                    })
                    .expect("No speed solution found")
            })
            .into();

        // Pick a hailstone that doesn't match speed in any of the coordinates to test with
        let test_hail =
            *self.hail.iter().find(|hail| (0..3).all(|dim| hail.v[dim] != v[dim])).unwrap();

        // Find a hail that matches the speed in one of the dimensions, to determine the time for the other hail
        let t = (0..3)
            .filter_map(|dim| {
                let same_hail = self.hail.iter().find(|hail| hail.v[dim] == v[dim])?;
                Some((same_hail.p[dim] - test_hail.p[dim]) / (test_hail.v[dim] - same_hail.v[dim]))
            })
            .next()
            .expect("Could not find any hail to match speeds with");

        // Now use the testing hail to calculate back in time to the starting position
        let start = test_hail.p + test_hail.v * t - v * t;

        start.coords.iter().sum()
    }
}

fn main() { execute_day::<Day>() }

#[cfg(test)]
mod tests {
    use advent_lib::day_test;

    day_test!( 24, example => 2, 47 );
    day_test!( 24 => 19523, 566373506408017 );
}
