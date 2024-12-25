#![feature(test)]

use advent_lib::day::*;
use advent_lib::grid::Grid;

#[derive(Debug)]
struct Day {
    locks: Vec<Vec<u32>>,
    keys: Vec<Vec<u32>>,
}

impl ExecutableDay for Day {
    type Output = usize;

    fn from_lines<LINES: Iterator<Item = String>>(lines: LINES) -> Self {
        let mut lines = lines.peekable();
        let mut locks = Vec::new();
        let mut keys = Vec::new();

        while lines.peek().is_some() {
            let grid = Grid::<u8>::from(lines.by_ref().take_while(|l| !l.is_empty()));
            if grid.east_line(0).all(|(_, &b)| b == b'#') {
                locks.push(
                    grid.x_range()
                        .map(|x| grid.south_line(x).take_while(|(_, &b)| b == b'#').count() as u32)
                        .collect(),
                )
            } else if grid.east_line(grid.height() - 1).all(|(_, &b)| b == b'#') {
                keys.push(
                    grid.x_range()
                        .map(|x| grid.north_line(x).take_while(|(_, &b)| b == b'#').count() as u32)
                        .collect(),
                )
            } else {
                println!("WARN unexpected grid: \n{:?}", grid);
            }
        }

        Day { locks, keys }
    }
    fn calculate_part1(&self) -> Self::Output {
        self.keys
            .iter()
            .flat_map(|key| self.locks.iter().map(move |lock| (key, lock)))
            .filter(|&(key, lock)| (0..key.len()).all(|index| key[index] + lock[index] <= 7))
            .count()
    }
    fn calculate_part2(&self) -> Self::Output { 0 }
}

fn main() { execute_day::<Day>() }

#[cfg(test)]
mod tests {
    use advent_lib::day_test;

    day_test!( 25, example1 => 3 );
    day_test!( 25 => 3021 );
}
