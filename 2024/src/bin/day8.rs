#![feature(test)]

use advent_lib::grid::{Grid, Location, Size};
use advent_lib::iter_utils::IteratorUtils;
use advent_lib::*;
use itertools::Itertools;
use nom_parse_macros::parse_from;
use std::collections::HashMap;

#[parse_from(Grid::parse)]
struct Field {
    grid: Grid<char>,
    #[derived(grid.entries().filter(|(_, &c)| c != '.').map(|(l, &c)| (c, l)).into_group_map())]
    antenna_locations: HashMap<char, Vec<Location>>,
}

impl Field {
    fn calculate_dips<G>(
        &self,
        iterations: G,
        start: Location,
        step: Size,
    ) -> impl Iterator<Item = Location> + use<'_, G>
    where
        G: IntoIterator<Item = i32>,
    {
        iterations
            .into_iter()
            .map(move |iteration| start + step * iteration)
            .take_while(|p| self.grid.is_valid_location(p))
    }

    fn count_dips(&self, iterations: impl IntoIterator<Item = i32> + Clone) -> usize {
        self.antenna_locations
            .values()
            .flat_map(|same_antenna_locations| {
                IteratorUtils::combinations(same_antenna_locations.iter()).flat_map(
                    |[&first, &second]| {
                        Iterator::chain(
                            self.calculate_dips(iterations.clone(), first, first - second),
                            self.calculate_dips(iterations.clone(), second, second - first),
                        )
                    },
                )
            })
            .unique()
            .count()
    }
}

fn calculate_part1(field: &Field) -> usize { field.count_dips([1]) }
fn calculate_part2(field: &Field) -> usize { field.count_dips(0..) }

day_main!(Field);
day_test!( 8, example1 => 14, 34 );
day_test!( 8 => 299, 1032);
