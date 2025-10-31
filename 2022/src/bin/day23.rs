#![feature(test)]

use advent_lib::direction::Direction;
use advent_lib::geometry::{point2, vector2, Vector};
use advent_lib::{
    grid::{Grid, Location},
    *,
};
use advent_macros::FromRepr;
use fxhash::FxHashMap;
use nom_parse_macros::parse_from;
use std::cmp::{max, min};

#[repr(u8)]
#[derive(FromRepr, PartialEq, Copy, Clone, Default)]
enum Ground {
    #[default]
    Empty = b'.',
    Elf = b'#',
}

#[parse_from(map({}, |grid: Grid<Ground>| {
    let mut elfs = Grid::new_empty(grid.width() * 3, grid.height() * 3);
    let offset = vector2(grid.width(), grid.height());
    for (loc, ground) in grid.entries() {
        if let Some(target) = elfs.get_mut(loc + offset) {
            *target = *ground;
        }
    }
    elfs
}))]
#[derive(Clone)]
struct Elfs(Grid<Ground>);

const DIRECTIONS: [(Direction, [usize; 3]); 4] = [
    (Direction::North, [7, 0, 1]),
    (Direction::South, [3, 4, 5]),
    (Direction::West, [5, 6, 7]),
    (Direction::East, [1, 2, 3]),
];

fn find_possible_move(round: usize, has_elfs: [&Ground; 8]) -> Option<Direction> {
    for ix in 0..4 {
        let (dir, ixs) = DIRECTIONS[(round + ix) % 4];
        if ixs.iter().all(|&ix| has_elfs[ix] == &Ground::Empty) {
            return Some(dir);
        }
    }
    None
}

fn step(elfs: &mut Elfs, round: usize) -> bool {
    let mut proposals = Vec::<(Location, Location)>::with_capacity(elfs.0.len());
    let mut proposal_count = FxHashMap::with_capacity_and_hasher(elfs.0.len(), Default::default());
    for (loc, _) in elfs.0.entries().filter(|&(_, g)| g == &Ground::Elf) {
        let neighbours = elfs.0.cardinal_neighbours(loc).unwrap();
        if neighbours.contains(&&Ground::Elf) {
            if let Some(dir) = find_possible_move(round, neighbours) {
                let target = loc + dir;
                proposals.push((loc, target));
                *proposal_count.entry(target).or_default() += 1;
            }
        }
    }

    let mut some_change = false;
    for &(current, target) in proposals.iter() {
        if proposal_count.get(&target) == Some(&1) {
            if let Some(ground) = elfs.0.get_mut(current) {
                *ground = Ground::Empty
            }
            if let Some(ground) = elfs.0.get_mut(target) {
                *ground = Ground::Elf
            }
            some_change = true;
        }
    }

    some_change
}

fn get_range(elfs: &Elfs) -> (Vector<2, i32>, Vector<2, i32>) {
    let middle = vector2(elfs.0.width() / 2, elfs.0.height() / 2);
    elfs.0.entries().filter(|&(_, ground)| ground == &Ground::Elf).fold(
        (middle, middle),
        |(low, high), (curr, _)| {
            (
                vector2(min(low.x(), curr.x()), min(low.y(), curr.y())),
                vector2(max(high.x(), curr.x()), max(high.y(), curr.y())),
            )
        },
    )
}

fn empty_fields(elfs: &Elfs) -> i32 {
    let (min, max) = get_range(elfs);
    let mut count = 0;
    for y in min.y()..=max.y() {
        for x in min.x()..=max.x() {
            if elfs.0.get(point2(x, y)) == Some(&Ground::Empty) {
                count += 1;
            }
        }
    }
    count
}

fn calculate_part1(elfs: &Elfs) -> i32 {
    let mut elfs = elfs.clone();
    for round in 0..10 {
        step(&mut elfs, round);
    }
    empty_fields(&elfs)
}

fn calculate_part2(elfs: &Elfs) -> usize {
    let mut elfs = elfs.clone();
    for round in 0.. {
        let change = step(&mut elfs, round);
        if !change {
            return round + 1;
        }
    }
    0
}

day_main!(Elfs);
day_test!( 23, example => 110, 20 );
day_test!( 23 => 3762, 997 );
