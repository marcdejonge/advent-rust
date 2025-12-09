#![feature(test)]

use advent_lib::{direction::Direction::*, grid::*, iter_utils::CountIf, *};
use advent_macros::FromRepr;

#[repr(u8)]
#[derive(FromRepr, PartialEq, Eq)]
enum Field {
    Start = b'S',
    Split = b'^',
    Empty = b'.',
    Beam = b'|',
}

struct Memoize(Grid<usize>);

impl Memoize {
    fn new(grid: &Grid<Field>) -> Self { Self(grid.map(|_| usize::MAX)) }

    fn split_count_from_start(&mut self, grid: &Grid<Field>) -> usize {
        self.split_count(grid, grid.find(|field| *field == Field::Start).unwrap())
    }

    fn split_count(&mut self, grid: &Grid<Field>, from: Location) -> usize {
        let mut cached = self.0.get(from).cloned().unwrap_or(0);
        if cached == usize::MAX {
            cached = match grid.get(from) {
                None => 0,
                Some(Field::Split) => {
                    self.split_count(grid, from + West) + self.split_count(grid, from + East) + 1
                }
                _ => self.split_count(grid, from + South),
            };
            *self.0.get_mut(from).unwrap() = cached;
        }
        cached
    }

    #[cfg(feature = "generate_image")]
    fn render_image(&self, filename: &str) {
        let max_value = self
            .0
            .entries()
            .map(|(_, v)| if *v == usize::MAX { 0 } else { *v })
            .max()
            .unwrap();
        let max_value = (max_value as f64).ln();

        self.0.render_to_image(filename, |count| {
            if *count == usize::MAX {
                [0, 0, 48, 255]
            } else if *count == 0 {
                [0, 0, 0, 255]
            } else {
                let v = ((*count as f64).ln() * 255. / max_value) as u8;
                [v, v, v, 255]
            }
        });
    }
}

fn calculate_part1(grid: &Grid<Field>) -> usize {
    let mut mem = Memoize::new(grid);
    mem.split_count_from_start(grid);
    grid.count_if(|&(loc, f)| f == &Field::Split && mem.0[loc] != usize::MAX)
}

fn calculate_part2(grid: &Grid<Field>) -> usize {
    let mut mem = Memoize::new(grid);
    let cnt = mem.split_count_from_start(grid);
    #[cfg(feature = "generate_image")]
    mem.render_image("day7.png");
    cnt + 1
}

day_main!(Grid<Field>);

day_test!( 7, example => 21, 40);
day_test!( 7 => 1585, 16716444407407 );
