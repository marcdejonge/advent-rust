/*
--- Day 13: Point of Incidence ---

With your help, the hot springs team locates an appropriate spring which launches you neatly and
precisely up to the edge of Lava Island.

There's just one problem: you don't see any lava.

You do see a lot of ash and igneous rock; there are even what look like gray mountains scattered
around. After a while, you make your way to a nearby cluster of mountains only to discover that the
valley between them is completely full of large mirrors. Most of the mirrors seem to be aligned in a
consistent way; perhaps you should head in that direction?

As you move through the valley of mirrors, you find that several of them have fallen from the large
metal frames keeping them in place. The mirrors are extremely flat and shiny, and many of the fallen
mirrors have lodged into the ash at strange angles. Because the terrain is all one color, it's hard
to tell where it's safe to walk or where you're about to run into a mirror.

You note down the patterns of ash (.) and rocks (#) that you see as you walk (your puzzle input);
perhaps by carefully analyzing these patterns, you can figure out where the mirrors are!

To find the reflection in each pattern, you need to find a perfect reflection across either a
horizontal line between two rows or across a vertical line between two columns.

To summarize your pattern notes, add up the number of columns to the left of each vertical line of
reflection; to that, also add 100 multiplied by the number of rows above each horizontal line of
reflection.

Find the line of reflection in each of the patterns in your notes. What number do you get after
summarizing all of your notes?

--- Part Two ---

You resume walking through the valley of mirrors and - SMACK! - run directly into one. Hopefully
nobody was watching, because that must have been pretty embarrassing.

Upon closer inspection, you discover that every mirror has exactly one smudge: exactly one . or #
should be the opposite type.

In each pattern, you'll need to locate and fix the smudge that causes a different reflection line to
be valid. (The old reflection line won't necessarily continue being valid after the smudge is fixed.)

In each pattern, fix the smudge and find the different line of reflection. What number do you get
after summarizing the new reflection line in each pattern in your notes?
*/
#![feature(test)]

use std::cmp::min;
use std::ops::Add;

use rayon::prelude::*;

use advent_lib::day::{execute_day, ExecutableDay};
use advent_lib::geometry::point2;
use advent_lib::grid::Grid;
use advent_macros::FromRepr;

struct Day {
    grids: Vec<Grid<Item>>,
}

#[repr(u8)]
#[derive(FromRepr, PartialEq, Copy, Clone)]
enum Item {
    None = b'.',
    Stone = b'#',
}

fn find_reflection(grid: &Grid<Item>, smudges: u32) -> i32 {
    for reflect_x in 1..grid.width() {
        let mut smudges_found = 0u32;
        'y: for y in grid.y_range() {
            let reflect_size = min(grid.width() - reflect_x, reflect_x);
            for dx in 0..reflect_size {
                let x_left = reflect_x - dx - 1;
                let x_right = reflect_x + dx;
                if grid.get(point2(x_left, y)) != grid.get(point2(x_right, y)) {
                    smudges_found += 1;
                    if smudges_found > smudges {
                        break 'y;
                    }
                }
            }
        }

        if smudges_found == smudges {
            return reflect_x;
        }
    }

    for reflect_y in 1..grid.height() {
        let mut smudges_found = 0u32;
        'x: for x in grid.x_range() {
            let reflect_size = min(grid.height() - reflect_y, reflect_y);
            for dy in 0..reflect_size {
                let y_left = reflect_y - dy - 1;
                let y_right = reflect_y + dy;
                if grid.get(point2(x, y_left)) != grid.get(point2(x, y_right)) {
                    smudges_found += 1;
                    if smudges_found > smudges {
                        break 'x;
                    }
                }
            }
        }

        if smudges_found == smudges {
            return reflect_y * 100;
        }
    }

    panic!("No solution found for:\n{grid:?}")
}

impl ExecutableDay for Day {
    type Output = i32;

    fn from_lines<LINES: Iterator<Item = String>>(lines: LINES) -> Self {
        let mut lines = lines.peekable();
        let mut grids = Vec::<Grid<_>>::new();
        while lines.peek().is_some() {
            grids.push(Grid::from(lines.by_ref().take_while(|s| !s.is_empty())));
        }
        Day { grids }
    }

    fn calculate_part1(&self) -> Self::Output {
        self.grids.par_iter().map(|g| find_reflection(g, 0)).reduce(|| 0, i32::add)
    }

    fn calculate_part2(&self) -> Self::Output {
        self.grids.par_iter().map(|g| find_reflection(g, 1)).reduce(|| 0, i32::add)
    }
}

fn main() { execute_day::<Day>() }

#[cfg(test)]
mod tests {
    use advent_lib::day_test;

    day_test!( 13, example => 405, 400 );
    day_test!( 13 => 37025, 32854 );

    mod find_reflection_tests {
        use advent_lib::grid::Grid;

        use crate::find_reflection;

        #[test]
        fn single() {
            let text = "#####..########\n##.######.####.\n.#.#.##.#.#..#.\n..###..###....#\n...##..##.....#\n####....#######\n#.#..##..#.##.#\n#...#..#...##..\n...######......\n.#.#....#.#..#.\n.###.##.###..##\n...######......\n###.####.######\n#.###..###.##.#\n#....##....##..\n.#........#..#.\n.#.#.##.#.#..#.";
            let grid = Grid::from(text.lines().map(str::to_owned));
            assert_eq!(6, find_reflection(&grid, 0));
            assert_eq!(12, find_reflection(&grid, 1));
        }

        #[test]
        fn test2() {
            let text =
                ".##.##.##..\n#.######.##\n.#..##..#..\n#.#.##.#.##\n#.#....#.##\n.#..##..###\n##..##..###\n##..##..###\n.#.####.#..\n#..####..##\n.#.#..#.#..\n.##.##.##..\n.##....##..";
            let grid = Grid::from(text.lines().map(str::to_owned));
            assert_eq!(10, find_reflection(&grid, 0));
            assert_eq!(5, find_reflection(&grid, 1));
        }
    }
}
