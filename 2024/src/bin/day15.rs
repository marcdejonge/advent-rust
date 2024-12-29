#![feature(test)]

use advent_lib::day_main;
use advent_lib::direction::Direction::*;
use advent_lib::direction::{direction_parser, Direction};
use advent_lib::geometry::point2;
use advent_lib::grid::{Grid, Location};
use advent_macros::{parsable, FromRepr};
use std::cmp::PartialEq;
use Block::*;

#[parsable(
    separated_pair(
        Grid::parser(),
        double_line_ending,
        map(
            separated_list1(line_ending, many1(direction_parser)),
            |commands| commands.into_iter().flatten().collect()
        )
    )
)]
struct Input {
    grid: Grid<Block>,
    commands: Vec<Direction>,
}

#[repr(u8)]
#[derive(Copy, Clone, FromRepr, PartialEq)]
enum Block {
    Empty = b'.',
    Wall = b'#',
    Box = b'O',
    LBox = b'[',
    RBox = b']',
    Robot = b'@',
}

fn can_move(grid: &Grid<Block>, loc: Location, dir: Direction) -> bool {
    let new_pos = loc + dir;
    match grid.get(new_pos) {
        Some(Empty) => true,
        Some(Box) => can_move(grid, new_pos, dir),
        Some(RBox) | Some(LBox) if dir.is_horizontal() => can_move(grid, new_pos + dir, dir),
        Some(RBox) => can_move(grid, new_pos, dir) && can_move(grid, new_pos + West, dir),
        Some(LBox) => can_move(grid, new_pos, dir) && can_move(grid, new_pos + East, dir),
        _ => false,
    }
}

fn move_block(grid: &mut Grid<Block>, loc: Location, dir: Direction) {
    let new_loc = loc + dir;
    match grid.get(new_loc) {
        Some(Empty) => grid.swap(loc, new_loc),
        Some(Box) => {
            move_block(grid, new_loc, dir);
            grid.swap(loc, new_loc);
        }
        Some(RBox) | Some(LBox) if dir.is_horizontal() => {
            move_block(grid, new_loc + dir, dir);
            grid.swap(new_loc, new_loc + dir);
            grid.swap(loc, new_loc);
        }
        Some(RBox) => {
            move_block(grid, new_loc, dir);
            move_block(grid, new_loc + West, dir);
            grid.swap(loc, new_loc);
        }
        Some(LBox) => {
            move_block(grid, new_loc, dir);
            move_block(grid, new_loc + East, dir);
            grid.swap(loc, new_loc);
        }
        _ => {}
    }
}

impl Input {
    fn execute_commands(&self, grid: &mut Grid<Block>) -> u32 {
        let mut pos = grid.find(|&b| b == Robot).unwrap();
        for &command in &self.commands {
            if can_move(grid, pos, command) {
                move_block(grid, pos, command);
                pos = pos + command;
            }
        }

        grid.entries()
            .filter(|(_, &b)| b == Box || b == LBox)
            .map(|(pos, _)| (pos.x() + pos.y() * 100) as u32)
            .sum()
    }
}

fn calculate_part1(input: &Input) -> u32 {
    let mut grid = input.grid.clone();
    input.execute_commands(&mut grid)
}

fn calculate_part2(input: &Input) -> u32 {
    let mut grid = Grid::new_default(Empty, input.grid.width() * 2, input.grid.height());
    input.grid.entries().for_each(|(pos, &b)| match b {
        Box => {
            grid[point2(pos.x() * 2, pos.y())] = LBox;
            grid[point2(pos.x() * 2 + 1, pos.y())] = RBox;
        }
        Robot => {
            grid[point2(pos.x() * 2, pos.y())] = Robot;
        }
        Wall => {
            grid[point2(pos.x() * 2, pos.y())] = Wall;
            grid[point2(pos.x() * 2 + 1, pos.y())] = Wall;
        }
        _ => {}
    });

    input.execute_commands(&mut grid)
}

day_main!();

#[cfg(test)]
mod tests {
    use advent_lib::day_test;

    day_test!( 15, example_small => 2028, 1751 );
    day_test!( 15, example_small2 => 908, 618 );
    day_test!( 15, example_bigger => 10092, 9021 );
    day_test!( 15 => 1421727, 1463160 );
}
