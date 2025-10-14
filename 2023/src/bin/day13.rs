#![feature(test)]

use advent_lib::geometry::point2;
use advent_lib::grid::Grid;
use advent_lib::parsing::double_line_ending;
use advent_lib::*;
use advent_macros::FromRepr;
use nom_parse_macros::parse_from;
use rayon::prelude::*;
use std::cmp::min;
use std::ops::Add;

#[parse_from(separated_list1(double_line_ending, Grid::parse))]
struct Grids(Vec<Grid<Item>>);

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

fn calculate_part1(grids: &Grids) -> i32 {
    grids.0.par_iter().map(|g| find_reflection(g, 0)).reduce(|| 0, i32::add)
}

fn calculate_part2(grids: &Grids) -> i32 {
    grids.0.par_iter().map(|g| find_reflection(g, 1)).reduce(|| 0, i32::add)
}

day_main!();
day_test!( 13, example => 405, 400 );
day_test!( 13 => 37025, 32854 );

#[cfg(test)]
mod find_reflection_tests {
    use crate::find_reflection;
    use advent_lib::grid::Grid;
    use nom::IResult;
    use nom_parse_trait::ParseFrom;

    #[test]
    fn single() {
        let text = b"#####..########\n##.######.####.\n.#.#.##.#.#..#.\n..###..###....#\n...##..##.....#\n####....#######\n#.#..##..#.##.#\n#...#..#...##..\n...######......\n.#.#....#.#..#.\n.###.##.###..##\n...######......\n###.####.######\n#.###..###.##.#\n#....##....##..\n.#........#..#.\n.#.#.##.#.#..#.";
        let grid: IResult<_, Grid<_>> = Grid::parse(text.as_ref());
        let grid = grid.unwrap().1;
        assert_eq!(6, find_reflection(&grid, 0));
        assert_eq!(12, find_reflection(&grid, 1));
    }

    #[test]
    fn test2() {
        let text =
                b".##.##.##..\n#.######.##\n.#..##..#..\n#.#.##.#.##\n#.#....#.##\n.#..##..###\n##..##..###\n##..##..###\n.#.####.#..\n#..####..##\n.#.#..#.#..\n.##.##.##..\n.##....##..";
        let grid: IResult<_, Grid<_>> = Grid::parse(text.as_ref());
        let grid = grid.unwrap().1;
        assert_eq!(10, find_reflection(&grid, 0));
        assert_eq!(5, find_reflection(&grid, 1));
    }
}
