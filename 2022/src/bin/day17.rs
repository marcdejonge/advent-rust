#![feature(test)]

use advent_lib::day::{execute_day, ExecutableDay};
use fxhash::FxBuildHasher;
use rusttype::{Point, Vector};
use std::collections::HashMap;

struct Day {
    moves: Vec<Move>,
}

type Move = Vector<i32>;
const L: Move = Vector { x: -1, y: 0 };
const R: Move = Vector { x: 1, y: 0 };
const D: Move = Vector { x: 0, y: -1 };

const SHAPES: [&[Vector<i32>]; 5] = [
    &[
        // Horizontal line
        Vector { x: 0, y: 0 },
        Vector { x: 1, y: 0 },
        Vector { x: 2, y: 0 },
        Vector { x: 3, y: 0 },
    ],
    &[
        // Plus
        Vector { x: 1, y: 0 },
        Vector { x: 0, y: 1 },
        Vector { x: 1, y: 1 },
        Vector { x: 2, y: 1 },
        Vector { x: 1, y: 2 },
    ],
    &[
        // Corner
        Vector { x: 0, y: 0 },
        Vector { x: 1, y: 0 },
        Vector { x: 2, y: 0 },
        Vector { x: 2, y: 1 },
        Vector { x: 2, y: 2 },
    ],
    &[
        // Vertical line
        Vector { x: 0, y: 0 },
        Vector { x: 0, y: 1 },
        Vector { x: 0, y: 2 },
        Vector { x: 0, y: 3 },
    ],
    &[
        // Block
        Vector { x: 0, y: 0 },
        Vector { x: 0, y: 1 },
        Vector { x: 1, y: 0 },
        Vector { x: 1, y: 1 },
    ],
];

const STEP_COUNT_1: usize = 2022;
const STEP_COUNT_2: usize = 1_000_000_000_000;

struct Field {
    rocks: Vec<[u8; 7]>,
    move_ix: usize,
}

impl Field {
    fn new() -> Field { Field { rocks: Vec::with_capacity(100), move_ix: 0 } }

    fn test_shape_fit(&self, shape: &[Vector<i32>], point: Point<i32>) -> bool {
        shape.iter().all(|sp| {
            let Point { x, y } = point + *sp;
            x >= 0
                && x < 7
                && y >= 0
                && ((y as usize) >= self.rocks.len() || self.rocks[y as usize][x as usize] == b'.')
        })
    }

    fn drop_rock(&mut self, moves: &Vec<Move>, shape: &[Vector<i32>]) {
        let mut position = Point::<i32> { x: 2, y: (self.rocks.len() as i32) + 3 };

        loop {
            let new_pos = position + moves[self.move_ix];
            self.move_ix = (self.move_ix + 1) % moves.len();

            if self.test_shape_fit(shape, new_pos) {
                position = new_pos;
            }

            let drop_pos = position + D;
            if !self.test_shape_fit(shape, drop_pos) {
                shape.iter().for_each(|v| {
                    let Point { x, y } = position + *v;
                    if self.rocks.len() as i32 <= y {
                        self.rocks.push([b'.'; 7])
                    }
                    self.rocks[y as usize][x as usize] = b'#';
                });
                break;
            }

            position = drop_pos;
        }
    }

    fn top_line(&self) -> [u8; 7] { self.rocks.last().unwrap().clone() }
}

#[derive(Hash, PartialEq, Eq)]
struct State {
    move_ix: usize,
    last_shape_ix: u8,
    last_line: [u8; 7],
}

impl ExecutableDay for Day {
    type Output = usize;

    fn from_lines<LINES: Iterator<Item = String>>(mut lines: LINES) -> Self {
        let moves = lines.next().unwrap().bytes().map(|b| if b == b'<' { L } else { R }).collect();
        Day { moves }
    }

    fn calculate_part1(&self) -> Self::Output {
        let mut field = Field::new();
        for ix in 0..STEP_COUNT_1 {
            field.drop_rock(&self.moves, SHAPES[ix % SHAPES.len()])
        }

        field.rocks.len()
    }

    fn calculate_part2(&self) -> Self::Output {
        let mut states = HashMap::<State, (usize, usize), FxBuildHasher>::default();
        let mut field = Field::new();

        for step in 0..usize::MAX {
            let next_rock = SHAPES[step % SHAPES.len()];
            field.drop_rock(&self.moves, next_rock);
            let new_state = State {
                move_ix: field.move_ix,
                last_shape_ix: (step % 5) as u8,
                last_line: field.top_line(),
            };

            if states.contains_key(&new_state) {
                let (last_height, last_step) = states.get(&new_state).unwrap();
                let cycle_steps = step - last_step;
                let cycle_count = (STEP_COUNT_2 - last_step) / cycle_steps;
                if cycle_count * cycle_steps + last_step == STEP_COUNT_2 {
                    return (cycle_count - 1) * (field.rocks.len() - last_height)
                        + (field.rocks.len() - 1);
                }
            }

            states.insert(new_state, (field.rocks.len(), step));
        }

        panic!("Not determined")
    }
}

fn main() { execute_day::<Day>() }

#[cfg(test)]
mod tests {
    use advent_lib::day_test;

    day_test!( 17, example => 3068, 1514285714288 );
    day_test!( 17 => 3098, 1525364431487 );
}
