#![feature(test)]
use advent_lib::day::{execute_day, ExecutableDay};
use fxhash::FxBuildHasher;
use rusttype::{Point, Vector};
use std::collections::HashSet;

struct Day {
    steps: Vec<(Vector<i32>, i32)>,
}

impl ExecutableDay for Day {
    type Output = usize;

    fn from_lines<LINES: Iterator<Item = String>>(lines: LINES) -> Self {
        Day { steps: lines.map(parse_line).collect() }
    }

    fn calculate_part1(&self) -> Self::Output { self.calculate_from([START; 2]) }

    fn calculate_part2(&self) -> Self::Output { self.calculate_from([START; 10]) }
}

const START: Point<i32> = Point { x: 0, y: 0 };
const R: Vector<i32> = Vector { x: 1, y: 0 };
const L: Vector<i32> = Vector { x: -1, y: 0 };
const U: Vector<i32> = Vector { x: 0, y: 1 };
const D: Vector<i32> = Vector { x: 0, y: -1 };

fn parse_line(line: String) -> (Vector<i32>, i32) {
    let (direction, count) = line.split_once(" ").expect("Missing space to split");
    let direction = match direction {
        "R" => R,
        "L" => L,
        "U" => U,
        "D" => D,
        _ => panic!("Unknown direction {}", direction),
    };
    (direction, count.parse().expect("Expected number"))
}

fn step_one_towards(orig: &Point<i32>, diff: &Vector<i32>) -> Point<i32> {
    let x = if diff.x == 0 { orig.x } else { orig.x + (diff.x / diff.x.abs()) };
    let y = if diff.y == 0 { orig.y } else { orig.y + (diff.y / diff.y.abs()) };
    Point { x, y }
}

fn move_snake<const N: usize>(snake: [Point<i32>; N], direction: Vector<i32>) -> [Point<i32>; N] {
    let mut result = [START; N];
    result[0] = *snake.first().expect("Cannot support empty snakes") + direction;
    for ix in 1..N {
        let diff = result[ix - 1] - snake[ix];
        result[ix] = if diff.x.abs() > 1 || diff.y.abs() > 1 {
            step_one_towards(&snake[ix], &diff)
        } else {
            snake[ix]
        };
    }
    result
}

impl Day {
    fn calculate_from<const N: usize>(&self, snake: [Point<i32>; N]) -> usize {
        let mut places = HashSet::with_hasher(FxBuildHasher::default());
        let mut snake = snake.clone();
        for &(direction, count) in &self.steps {
            for _ in 0..count {
                snake = move_snake(snake, direction);
                places.insert(snake[N - 1]);
            }
        }
        places.len()
    }
}

fn main() { execute_day::<Day>() }

#[cfg(test)]
mod tests {
    use advent_lib::day_test;
    day_test!( 9, example => 13, 1 );
    day_test!( 9, bigger => 88, 36 );
    day_test!( 9 => 5710, 2259 );
}
