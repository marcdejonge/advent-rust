#![feature(test)]

use advent_lib::{
    geometry::{point2, vector2},
    grid::{Grid, Location},
    parsing::single_digit,
    search::a_star_search,
    *,
};
use nom_parse_macros::parse_from;

#[derive(Clone, Copy, Default)]
#[parse_from(map(single_digit(), |b| (b - b'0') as i32))]
struct Field(i32);

impl From<Field> for char {
    fn from(value: Field) -> Self {
        value.0.to_string().as_bytes()[0] as char
    }
}

fn calculate_score(grid: &Grid<Field>) -> i32 {
    let start = point2(0, 0);
    let goal = Location::from(grid.size() - vector2(1, 1));
    let graph = grid.search_graph(goal, |_, _, val| Some(val.0), |vec| vec.euler());

    let mut path = a_star_search(&graph, start).unwrap();
    path.pop(); // Drop the start node
    path.iter().map(|&loc| grid.get(loc).unwrap().0).sum()
}

fn calculate_part1(grid: &Grid<Field>) -> i32 {
    calculate_score(grid)
}

fn calculate_part2(grid: &Grid<Field>) -> i32 {
    let mut big_grid = Grid::<Field>::new_empty(grid.width() * 5, grid.height() * 5);
    for dy in 0..5 {
        for dx in 0..5 {
            let loc_add = vector2(grid.width() * dx, grid.height() * dy);
            let val_add = dx + dy;
            grid.entries().for_each(|(loc, val)| {
                let mut val = val.0 + val_add;
                if val > 9 {
                    val -= 9;
                }
                *big_grid.get_mut(loc + loc_add).unwrap() = Field(val)
            });
        }
    }
    calculate_score(&big_grid)
}

day_main!(Grid<Field>);

day_test!( 15, example => 40, 315 );
day_test!( 15 => 609, 2925 );
