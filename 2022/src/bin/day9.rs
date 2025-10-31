#![feature(test)]

use advent_lib::direction::Direction;
use advent_lib::geometry::{point2, Point, Vector};
use advent_lib::parsing::single_space;
use advent_lib::*;
use fxhash::FxBuildHasher;
use nom_parse_macros::parse_from;
use std::collections::HashSet;

#[parse_from(separated_list1(line_ending, separated_pair(Direction::parse, single_space(), i32)))]
struct Steps(Vec<(Direction, i32)>);

fn calculate_part1(steps: &Steps) -> usize { steps.calculate_from([START; 2]) }

fn calculate_part2(steps: &Steps) -> usize { steps.calculate_from([START; 10]) }

const START: Point<2, i32> = point2(0, 0);

fn step_one_towards(orig: &Point<2, i32>, diff: &Vector<2, i32>) -> Point<2, i32> {
    let x = if diff.x() == 0 { orig.x() } else { orig.x() + (diff.x() / diff.x().abs()) };
    let y = if diff.y() == 0 { orig.y() } else { orig.y() + (diff.y() / diff.y().abs()) };
    point2(x, y)
}

fn move_snake<const N: usize>(
    snake: [Point<2, i32>; N],
    direction: Vector<2, i32>,
) -> [Point<2, i32>; N] {
    let mut result = [START; N];
    result[0] = *snake.first().expect("Cannot support empty snakes") + direction;
    for ix in 1..N {
        let diff = result[ix - 1] - snake[ix];
        result[ix] = if diff.x().abs() > 1 || diff.y().abs() > 1 {
            step_one_towards(&snake[ix], &diff)
        } else {
            snake[ix]
        };
    }
    result
}

impl Steps {
    fn calculate_from<const N: usize>(&self, snake: [Point<2, i32>; N]) -> usize {
        let mut places = HashSet::with_hasher(FxBuildHasher::default());
        let mut snake = snake;
        for &(direction, count) in &self.0 {
            for _ in 0..count {
                snake = move_snake(snake, direction.as_vec());
                places.insert(snake[N - 1]);
            }
        }
        places.len()
    }
}

day_main!(Steps);
day_test!( 9, example => 13, 1 );
day_test!( 9, bigger => 88, 36 );
day_test!( 9 => 5710, 2259 );
