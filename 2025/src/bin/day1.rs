#![feature(test)]

use advent_lib::*;
use nom_parse_macros::parse_from;

#[parse_from]
enum Turn {
    #[format("L")]
    Left,
    #[format("R")]
    Right,
}

#[parse_from(({},{}))]
struct Action {
    turn: Turn,
    steps: u32,
}

#[derive(Debug, Copy, Clone)]
struct Position(i64);

impl Position {
    fn new() -> Self { Position(50) }

    fn apply(&mut self, action: &Action) -> u64 {
        let mut clicks = (action.steps / 100) as u64;
        let steps = (action.steps % 100) as i64;

        if steps > 0 {
            match action.turn {
                Turn::Left if self.0 > steps => self.0 -= steps,
                Turn::Left if self.0 == steps => {
                    self.0 = 0;
                    // We'll end up at 0, already count it as a click
                    clicks += 1;
                }
                Turn::Left => {
                    // Only count a click if we are not already at 0
                    if self.0 != 0 {
                        clicks += 1;
                    }
                    self.0 += 100 - steps;
                }
                Turn::Right => {
                    self.0 += steps;
                    if self.0 >= 100 {
                        self.0 -= 100;
                        clicks += 1;
                    }
                }
            }
        }
        clicks
    }
}

fn calculate_part1(input: &[Action]) -> usize {
    input
        .iter()
        .scan(Position::new(), |pos, action| {
            pos.apply(action);
            Some(pos.clone())
        })
        .filter(|pos| pos.0 == 0)
        .count()
}

fn calculate_part2(input: &[Action]) -> u64 {
    input.iter().scan(Position::new(), |pos, action| Some(pos.apply(action))).sum()
}

day_main!(Vec<Action>);

day_test!( 1, example => 3, 6 );
day_test!( 1 => 1086, 6268 );
