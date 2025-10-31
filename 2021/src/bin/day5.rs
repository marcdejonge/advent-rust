#![feature(test)]

use advent_lib::{
    geometry::{Point, Vector},
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
        line: *line,
        next: Some(line.start),
        step: (line.end - line.start).signum(),
    }
}

fn is_horizontal_or_vertical(line: &Line) -> bool {
    line.start.x() == line.end.x() || line.start.y() == line.end.y()
}

fn write_line_to(grid: &mut Grid<u8>, line: &LineSegment<2, i32>) {
    for point in iter(line) {
        if let Some(e) = grid.get_mut(point) {
            *e += 1;
        }
    }
}

fn calculate_part1(input: &[Line]) -> u64 {
    let mut grid = Grid::<u8>::new_empty(
        input.iter().map(|l| l.max_x()).max().unwrap() + 1,
        input.iter().map(|l| l.max_y()).max().unwrap() + 1,
    );
    for line in input {
        if is_horizontal_or_vertical(line) {
            write_line_to(&mut grid, line);
        }
    }
    grid.entries().filter(|(_, &value)| value >= 2).count() as u64
}

fn calculate_part2(input: &[Line]) -> u64 {
    let mut grid = Grid::<u8>::new_empty(
        input.iter().map(|l| l.max_x()).max().unwrap() + 1,
        input.iter().map(|l| l.max_y()).max().unwrap() + 1,
    );
    for line in input {
        write_line_to(&mut grid, line);
    }
    grid.entries().filter(|(_, &value)| value >= 2).count() as u64
}

day_main!(Vec<Line>);

day_test!( 5, example => 5, 12 );
day_test!( 5 => 4728, 17717 );
