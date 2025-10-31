#![feature(test)]

use advent_lib::direction::Direction;
use advent_lib::geometry::{point2, vector2};
use advent_lib::*;
use fxhash::FxBuildHasher;
use nom_parse_macros::parse_from;
use std::collections::HashMap;

#[parse_from(many1(Direction::parse))]
struct Moves(Vec<Direction>);

type Vector = advent_lib::geometry::Vector<2, i32>;
type Point = advent_lib::geometry::Point<2, i32>;

const SHAPES: [&[Vector]; 5] = [
    &[
        // Horizontal line
        vector2(0, 0),
        vector2(1, 0),
        vector2(2, 0),
        vector2(3, 0),
    ],
    &[
        // Plus
        vector2(1, 0),
        vector2(0, 1),
        vector2(1, 1),
        vector2(2, 1),
        vector2(1, 2),
    ],
    &[
        // Corner
        vector2(0, 0),
        vector2(1, 0),
        vector2(2, 0),
        vector2(2, 1),
        vector2(2, 2),
    ],
    &[
        // Vertical line
        vector2(0, 0),
        vector2(0, 1),
        vector2(0, 2),
        vector2(0, 3),
    ],
    &[
        // Block
        vector2(0, 0),
        vector2(0, 1),
        vector2(1, 0),
        vector2(1, 1),
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

    fn test_shape_fit(&self, shape: &[Vector], point: Point) -> bool {
        shape.iter().all(|sp| {
            let [x, y] = (point + *sp).coords;
            (0..7).contains(&x)
                && y >= 0
                && ((y as usize) >= self.rocks.len() || self.rocks[y as usize][x as usize] == b'.')
        })
    }

    fn drop_rock(&mut self, moves: &[Direction], shape: &[Vector]) {
        let mut position = point2(2, (self.rocks.len() as i32) + 3);

        loop {
            let new_pos = position + moves[self.move_ix];
            self.move_ix = (self.move_ix + 1) % moves.len();

            if self.test_shape_fit(shape, new_pos) {
                position = new_pos;
            }

            let drop_pos = position + Direction::North;
            if !self.test_shape_fit(shape, drop_pos) {
                shape.iter().for_each(|v| {
                    let [x, y] = (position + *v).coords;
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

    fn top_line(&self) -> [u8; 7] { *self.rocks.last().unwrap() }
}

#[derive(Hash, PartialEq, Eq)]
struct State {
    move_ix: usize,
    last_shape_ix: u8,
    last_line: [u8; 7],
}

fn calculate_part1(moves: &Moves) -> usize {
    let mut field = Field::new();
    for ix in 0..STEP_COUNT_1 {
        field.drop_rock(&moves.0, SHAPES[ix % SHAPES.len()])
    }

    field.rocks.len()
}

fn calculate_part2(moves: &Moves) -> usize {
    let mut states = HashMap::<State, (usize, usize), FxBuildHasher>::default();
    let mut field = Field::new();

    for step in 0..usize::MAX {
        let next_rock = SHAPES[step % SHAPES.len()];
        field.drop_rock(&moves.0, next_rock);
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

day_main!(Moves);
day_test!( 17, example => 3068, 1514285714288 );
day_test!( 17 => 3098, 1525364431487 );
