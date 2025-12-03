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
struct Turn(i64);

fn calculate_part1(turns: &[Turn]) -> usize {
    turns
        .iter()
        .scan(50, |pos, Turn(turn)| {
            *pos += turn % 100;
            if *pos < 0 {
                *pos += 100;
            } else if *pos >= 100 {
                *pos -= 100;
            }
            Some(*pos)
        })
        .filter(|pos| *pos == 0)
        .count()
}

fn calculate_part2(turns: &[Turn]) -> u64 {
    turns
        .iter()
        .scan(50, |pos, Turn(turn)| {
            let mut clicks = (turn / 100).unsigned_abs();
            let steps = turn % 100;

            if steps != 0 {
                *pos += steps;
                if *pos < 0 {
                    // Only count a click if we were not already at 0
                    if *pos != steps {
                        clicks += 1;
                    }
                    *pos += 100;
                } else if *pos >= 100 {
                    *pos -= 100;
                    clicks += 1;
                } else if *pos == 0 {
                    clicks += 1; // Always count a click immediately reaching 0
                }
            }

            Some(clicks)
        })
        .sum()
}

day_main!(Vec<Turn>);

day_test!( 1, example => 3, 6 );
day_test!( 1 => 1086, 6268 );
