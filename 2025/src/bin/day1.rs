#![feature(test)]

use advent_lib::*;
use nom_parse_macros::parse_from;

#[parse_from(map((one_of("LR"), u32), |(turn, steps)| {
    match turn {
      'L' => -(steps as i64),
      'R' => steps as i64,
      _    => unreachable!(),
    }
}))]
struct Action(i64);

impl Action {
    fn apply_to(&self, nr: &mut i64) -> u64 {
        let mut clicks = (self.0 / 100).abs() as u64;
        let steps = self.0 % 100;

        if steps != 0 {
            *nr += steps;
            if *nr < 0 {
                // Only count a click if we were not already at 0
                if *nr != steps {
                    clicks += 1;
                }
                *nr += 100;
            } else if *nr >= 100 {
                *nr -= 100;
                clicks += 1;
            } else if *nr == 0 {
                clicks += 1; // Always count a click immediately reaching 0
            }
        }

        clicks
    }
}

fn calculate_part1(input: &[Action]) -> usize {
    input
        .iter()
        .scan(50, |pos, action| {
            action.apply_to(pos);
            Some(*pos)
        })
        .filter(|pos| *pos == 0)
        .count()
}

fn calculate_part2(input: &[Action]) -> u64 {
    input.iter().scan(50, |pos, action| Some(action.apply_to(pos))).sum()
}

day_main!(Vec<Action>);

day_test!( 1, example => 3, 6 );
day_test!( 1 => 1086, 6268 );
