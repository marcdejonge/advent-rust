#![feature(test)]
#![feature(iter_map_windows)]

use advent_lib::builder::with_default;
use advent_lib::day::*;
use advent_lib::geometry::{point2, vector2, Vector};
use advent_lib::grid::{Grid, Location};
use advent_lib::small_string::SmallString;
use fxhash::FxHashMap;
use nom::character::complete::{alphanumeric1, line_ending};
use nom::combinator::map;
use nom::error::Error;
use nom::multi::separated_list1;
use nom::Parser;
use smallvec::{smallvec, SmallVec};

struct Day {
    lines: Vec<SmallString<8>>,
}

type MoveString = SmallString<8>;

#[derive(Debug, Default)]
struct Moves {
    move_cache: FxHashMap<(u8, u8), SmallVec<[MoveString; 2]>>,
    size_cache: FxHashMap<(MoveString, u8), usize>,
    positions: FxHashMap<u8, Location>,
}

impl Moves {
    fn register_locations<const D: usize>(&mut self, grid: [&str; D], offset: Vector<2, i32>) {
        Grid::from(grid.iter().map(|s| s.to_string()))
            .entries()
            .for_each(|(point, &c)| {
                self.positions.insert(c, point + offset);
            });
    }

    fn new() -> Self {
        with_default(|moves: &mut Self| {
            moves.register_locations(["789", "456", "123", " 0A"], vector2(0, 0));
            moves.register_locations([" ^A", "<v>"], vector2(0, 3));
        })
    }

    fn generate_move_options(&self, from: u8, to: u8) -> SmallVec<[MoveString; 2]> {
        let from_pos = self.positions[&from];
        let to_pos = self.positions[&to];

        let h_move = if from_pos.x() < to_pos.x() { b'>' } else { b'<' };
        let h_cnt = (from_pos.x() - to_pos.x()).unsigned_abs() as usize;
        let v_move = if from_pos.y() < to_pos.y() { b'v' } else { b'^' };
        let v_cnt = (from_pos.y() - to_pos.y()).unsigned_abs() as usize;

        if h_cnt == 0 {
            smallvec![SmallString::new().repeat(v_move, v_cnt).close()]
        } else if v_cnt == 0 {
            smallvec![SmallString::new().repeat(h_move, h_cnt).close()]
        } else {
            let mut result = SmallVec::new();
            // Test if moving vertical first doesn't cross the empty space
            if point2(from_pos.x(), to_pos.y()) != self.positions[&b' '] {
                result.push(SmallString::new().repeat(v_move, v_cnt).repeat(h_move, h_cnt).close());
            }
            // Test if moving horizontal first doesn't cross the empty space
            if point2(to_pos.x(), from_pos.y()) != self.positions[&b' '] {
                result.push(SmallString::new().repeat(h_move, h_cnt).repeat(v_move, v_cnt).close());
            }
            result
        }
    }

    fn get_move_options(&mut self, from: u8, to: u8) -> SmallVec<[MoveString; 2]> {
        if let Some(cached) = self.move_cache.get(&(from, to)) {
            return cached.clone();
        }
        let result = self.generate_move_options(from, to);
        self.move_cache.insert((from, to), result.clone());
        result
    }

    fn calc(&mut self, line: MoveString, depth: u8) -> usize {
        if depth == 0 {
            return line.len();
        }

        if let Some(cached) = self.size_cache.get(&(line.clone(), depth)) {
            return *cached;
        }

        let (result, _) = line.iter().fold((0, b'A'), |(last_count, last), &c| {
            let count = self
                .get_move_options(last, c)
                .into_iter()
                .map(|s| self.calc(s, depth - 1))
                .min()
                .unwrap();
            (count + last_count, c)
        });

        self.size_cache.insert((line, depth), result);
        result
    }

    fn score(&mut self, line: MoveString, depth: u8) -> usize {
        let digits = &line.as_bytes()[0..(line.len() - 1)];
        let nr: usize = String::from_utf8_lossy(digits).parse().unwrap();
        self.calc(line.clone(), depth + 1) * nr
    }
}

impl ExecutableDay for Day {
    type Output = usize;

    fn day_parser<'a>() -> impl Parser<&'a [u8], Self, Error<&'a [u8]>> {
        map(
            separated_list1(line_ending, alphanumeric1),
            |lines: Vec<&[u8]>| Day { lines: lines.into_iter().map(SmallString::from).collect() },
        )
    }

    fn calculate_part1(&self) -> Self::Output {
        let mut moves = Moves::new();
        self.lines.iter().map(|line| moves.score(line.clone(), 2)).sum()
    }
    fn calculate_part2(&self) -> Self::Output {
        let mut moves = Moves::new();
        self.lines.iter().map(|line| moves.score(line.clone(), 25)).sum()
    }
}

fn main() { execute_day::<Day>() }

#[cfg(test)]
mod tests {
    use advent_lib::day_test;

    day_test!( 21, example1 => 126384 );
    day_test!( 21 => 242484, 294209504640384 );
}
