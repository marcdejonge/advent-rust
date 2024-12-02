#![feature(test)]

use advent_lib::day::*;
use fxhash::FxHashMap;
use rayon::prelude::*;

struct Day {
    left: Vec<i64>,
    right: Vec<i64>,
}

impl Day {
    fn new() -> Self { Day { left: Vec::new(), right: Vec::new() } }
}

impl ExecutableDay for Day {
    type Output = i64;

    fn from_lines<LINES: Iterator<Item = String>>(lines: LINES) -> Self {
        let mut day = Day::new();

        for line in lines {
            let mut split = line.split_whitespace();
            day.left.push(split.next().unwrap().parse().unwrap());
            day.right.push(split.next().unwrap().parse().unwrap());
        }

        day.left.sort();
        day.right.sort();

        day
    }
    fn calculate_part1(&self) -> Self::Output {
        self.left.iter().zip(self.right.iter()).map(|(l, r)| (l - r).abs()).sum()
    }
    fn calculate_part2(&self) -> Self::Output {
        let mut map = FxHashMap::default();
        self.right.iter().for_each(|r| *map.entry(r).or_insert(0) += 1);
        self.left.par_iter().map(|l| map.get(l).unwrap_or(&0) * l).sum()
    }
}

fn main() { execute_day::<Day>() }

#[cfg(test)]
mod tests {
    use advent_lib::day_test;

    day_test!( 1, example1 => 11, 31 );
    day_test!( 1 => 1889772, 23228917);
}
