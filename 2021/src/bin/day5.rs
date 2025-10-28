#![feature(test)]

use advent_lib::{
    geometry::{vector2, Point, Vector},
    grid::Grid,
    lines::LineSegment,
    *,
};

type Line = LineSegment<2, i32>;

struct LineIterator {
    line: Line,
    next: Option<Point<2, i32>>,
    step: Vector<2, i32>,
}

impl Iterator for LineIterator {
    type Item = Point<2, i32>;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.next?;
        if current == self.line.end {
            self.next = None;
        } else {
            self.next = Some(current + self.step);
        }
        Some(current)
    }
}

fn iter(line: &Line) -> LineIterator {
    LineIterator {
        line: line.clone(),
        next: Some(line.start.clone()),
        step: {
            let direction = line.end - line.start;
            vector2(direction.x().signum(), direction.y().signum())
        },
    }
}

fn is_horizontal_or_vertical(line: &Line) -> bool {
    line.start.x() == line.end.x() || line.start.y() == line.end.y()
}

fn calculate_part1(input: &Vec<Line>) -> u64 {
    let mut grid = Grid::<u8>::new_empty(1000, 1000);
    for line in input {
        if is_horizontal_or_vertical(line) {
            for point in iter(line) {
                grid.get_mut(point).map(|e| *e += 1);
            }
        }
    }
    grid.entries().filter(|(_, &value)| value >= 2).count() as u64
}

fn calculate_part2(input: &Vec<Line>) -> u64 {
    let mut grid = Grid::<u8>::new_empty(1000, 1000);
    for line in input {
        for point in iter(line) {
            grid.get_mut(point).map(|e| *e += 1);
        }
    }
    grid.entries().filter(|(_, &value)| value >= 2).count() as u64
}

day_main!();

day_test!( 5, example => 5, 12 );
day_test!( 5 => 4728, 17717 );
