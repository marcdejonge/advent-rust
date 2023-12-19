#![feature(test)]

use std::cmp::min;
use std::ops::Add;

use rayon::prelude::*;

use advent_lib::day::*;
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
