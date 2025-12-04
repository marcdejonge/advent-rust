#![feature(test)]

use advent_lib::{
    direction::CardinalDirection,
    geometry::{Point, Vector},
    grid::Grid,
    *,
};
use advent_macros::FromRepr;

#[repr(u8)]
#[derive(FromRepr, Copy, Clone, Default, Eq, PartialEq, Debug)]
enum Place {
    #[default]
    None = b'.',
    Roll = b'@',
}

fn can_be_removed(grid: &Grid<Place>) -> impl Fn(&(Point<2, i32>, &Place)) -> bool {
    move |&(loc, place)| {
        place == &Place::Roll
            && CardinalDirection::ALL
                .into_iter()
                .filter(|&dir| grid.get(loc + Vector::from(dir)) == Some(&Place::Roll))
                .count()
                < 4
    }
}

fn calculate_part1(grid: &Grid<Place>) -> usize {
    grid.entries().filter(can_be_removed(grid)).count()
}

fn calculate_part2(grid: &Grid<Place>) -> usize {
    let mut removed = 0;
    let mut grid = grid.clone();
    loop {
        let remove_locs: Vec<_> =
            grid.entries().filter(can_be_removed(&grid)).map(|(loc, _)| loc).collect();
        if remove_locs.is_empty() {
            break;
        }
        removed += remove_locs.len();
        for loc in remove_locs {
            *grid.get_mut(loc).unwrap() = Place::None;
        }
    }
    removed
}

day_main!(Grid<Place>);

day_test!( 4, example => 13, 43 );
day_test!( 4 => 1508, 0 );
