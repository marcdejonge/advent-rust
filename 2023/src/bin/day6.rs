#![feature(test)]

use advent_lib::*;
use nom_parse_macros::parse_from;

#[parse_from(
    map(
        separated_pair(
            preceded(("Time:", space1), separated_list1(space1, u64)),
            line_ending,
            preceded(
                ("Distance:", space1),
                separated_list1(space1, u64),
            ),
        ),
        |(times, distances)| times.iter().zip(distances.iter())
                    .map(|(&time, &distance)| Race { time, distance })
                    .collect()
    )
)]
struct Input {
    races: Vec<Race>,
}

#[derive(Debug, Default, Copy, Clone)]
struct Race {
    time: u64,
    distance: u64,
}

impl Race {
    fn solve(&self) -> u64 {
        let time = self.time as f64;
        let dist = self.distance as f64;
        let s = (time * time - 4f64 * dist).sqrt();
        let min = (time - s) / 2f64;
        let max = (time + s) / 2f64;
        let min = if min.ceil() == min { min as u64 + 1 } else { min.ceil() as u64 };
        let max = if max.floor() == max { max as u64 - 1 } else { max.floor() as u64 };
        max - min + 1
    }

    fn combine(self, other: &Race) -> Race {
        Race {
            time: (self.time.to_string() + &other.time.to_string()).parse().unwrap(),
            distance: (self.distance.to_string() + &other.distance.to_string()).parse().unwrap(),
        }
    }
}

fn calculate_part1(input: &Input) -> u64 { input.races.iter().map(Race::solve).product() }

fn calculate_part2(input: &Input) -> u64 {
    input.races.iter().fold(Default::default(), Race::combine).solve()
}

day_main!();
day_test!( 6, example => 288, 71503);
day_test!( 6 => 1731600, 40087680 );
