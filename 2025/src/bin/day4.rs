#![feature(test)]

use advent_lib::{grid::Grid, iter_utils::CountIf, *};
use advent_macros::FromRepr;
use fxhash::FxHashMap;
use std::collections::hash_map::Entry;

#[repr(u8)]
#[derive(FromRepr, Copy, Clone, Default, Eq, PartialEq, Debug)]
enum Place {
    #[default]
    None = b'.',
    Roll = b'@',
}

fn calculate_part1(grid: &Grid<Place>) -> usize {
    grid.locations_where(|p| p == &Place::Roll)
        .filter(|loc| {
            loc.cardinal_neighbours().count_if(|&p| grid.get(p) == Some(&Place::Roll)) < 4
        })
        .count()
}

fn calculate_part2(grid: &Grid<Place>) -> usize {
    let mut locations: FxHashMap<_, _> = grid
        .locations_where(|p| p == &Place::Roll)
        .map(|loc| {
            (
                loc,
                loc.cardinal_neighbours().count_if(|&p| grid.get(p) == Some(&Place::Roll)),
            )
        })
        .collect();

    // Start by finding the first round of locations to remove
    let mut locs_to_remove: Vec<_> =
        locations.iter().filter(|&(_, count)| *count < 4).map(|(loc, _)| *loc).collect();

    let before_count = locations.len();

    while let Some(next) = locs_to_remove.pop() {
        locations.remove(&next);
        for nb in next.cardinal_neighbours() {
            if let Entry::Occupied(mut entry) = locations.entry(nb) {
                let count = entry.get_mut();
                *count -= 1;
                if *count == 3 {
                    locs_to_remove.push(*entry.key());
                }
            }
        }
    }

    before_count - locations.len()
}

day_main!(Grid<Place>);

day_test!( 4, example => 13, 43 );
day_test!( 4 => 1508, 8538 );
