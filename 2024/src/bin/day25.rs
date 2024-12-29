#![feature(test)]

use advent_lib::day_main;
use advent_lib::grid::Grid;
use advent_macros::parsable;

#[derive(Debug)]
#[parsable(separated_double_lines1())]
struct LocksAndKeys {
    #[defer(it.iter()
        .filter(|grid: &&Grid<u8>| grid.east_line(0).all(|(_, &b)| b == b'#'))
        .map(|grid| grid.x_range()
            .map(|x| grid.south_line(x).take_while(|(_, &b)| b == b'#').count())
            .collect())
        .collect()
    )]
    locks: Vec<Vec<usize>>,
    #[defer(it.iter()
        .filter(|grid: &&Grid<u8>| grid.east_line(grid.height() - 1).all(|(_, &b)| b == b'#'))
        .map(|grid| grid.x_range()
            .map(|x| grid.north_line(x).take_while(|(_, &b)| b == b'#').count())
            .collect())
        .collect()
    )]
    keys: Vec<Vec<usize>>,
}

fn calculate_part1(input: &LocksAndKeys) -> usize {
    input
        .keys
        .iter()
        .flat_map(|key| input.locks.iter().map(move |lock| (key, lock)))
        .filter(|&(key, lock)| (0..key.len()).all(|index| key[index] + lock[index] <= 7))
        .count()
}

day_main!(calculate_part1);

#[cfg(test)]
mod tests {
    use advent_lib::day_test;

    day_test!( 25, example1 => 3 );
    day_test!( 25 => 3021 );
}
