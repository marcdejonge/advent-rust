use advent_lib::day::{execute_day, ExecutableDay};
use advent_lib::direction::Direction;
use advent_lib::geometry::{point2, vector2, Point, Vector};
use advent_lib::grid::Grid;
use advent_macros::FromRepr;
use Direction::*;

use crate::FieldType::*;

#[derive(Debug, PartialEq)]
enum Command {
    Forward { steps: u32 },
    Left,
    Right,
}

struct Person {
    position: Point<2, u64>,
    direction: Direction,
}

impl Person {
    fn score(&self) -> u64 {
        (self.position.x() + 1) * 4
            + (self.position.y() + 1) * 1000
            + match self.direction {
                East => 0,
                South => 1,
                West => 2,
                North => 3,
            }
    }
}

fn parse_commands(input: &str) -> Result<Vec<Command>, String> {
    let mut commands = Vec::<Command>::with_capacity(input.len());
    let mut number = 0u32;

    for b in input.bytes() {
        if b.is_ascii_digit() {
            number = number * 10 + (b - b'0') as u32
        } else {
            commands.push(Command::Forward { steps: number });
            number = 0;
            match b {
                b'L' => commands.push(Command::Left),
                b'R' => commands.push(Command::Right),
                _ => return Err(format!("Unexpected character {b}")),
            }
        }
    }

    if number > 0 {
        commands.push(Command::Forward { steps: number })
    }

    Ok(commands)
}

#[repr(u8)]
#[derive(FromRepr, Default, Clone, Copy, PartialEq)]
enum FieldType {
    #[default]
    Outside = b' ',
    Empty = b'.',
    Wall = b'#',
}
struct Day {
    commands: Vec<Command>,
    block_size: usize,
    blocks: Vec<Block>,
}

struct Block {
    offset: Vector<2, i32>,
    grid: Grid<FieldType>,
}

impl ExecutableDay for Day {
    type Output = u64;

    fn from_lines<LINES: Iterator<Item = String>>(mut lines: LINES) -> Self {
        let mut field_lines: Vec<String> =
            lines.by_ref().take_while(|line| !line.is_empty()).collect();
        let width = field_lines.iter().map(|line| line.len()).max().unwrap_or(0);
        field_lines.iter_mut().for_each(|line| *line = format!("{:width$}", line));

        let field = Grid::from(field_lines.into_iter());
        let commands = parse_commands(lines.next().unwrap().as_str()).unwrap();

        let mut blocks = Vec::new();
        let bsize = field
            .y_range()
            .map(|y| point2(0, y))
            .map(|start| field.iter_line(start, East.as_vec()).filter(|f| **f != Outside).count())
            .min()
            .unwrap();

        for y in field.y_range().step_by(bsize) {
            for x in field.x_range().step_by(bsize) {
                if field[point2(x, y)] != Outside {
                    blocks.push(Block {
                        offset: vector2(x, y),
                        grid: field.sub_grid(x..(x + bsize as i32), y..(y + bsize as i32)),
                    })
                }
            }
        }

        Day { commands, block_size: bsize, blocks }
    }

    fn calculate_part1(&self) -> Self::Output {
        let neighbours = Vec::<[usize; 4]>::with_capacity(self.blocks.len());

        // First horizontal find of neighbouring blocks
        for curr_ix in 0..self.blocks.len() {
            let offset = self.blocks[curr_ix].offset;

            let block_iter = self.blocks.iter().enumerate();

            let north_ix = block_iter
                .clone()
                .filter(|(_, b)| b.offset.x() == offset.x() && b.offset.y() < offset.y())
                .map(|(ix, _)| ix)
                .last()
                .or(block_iter
                    .clone()
                    .filter(|(_, b)| b.offset.x() == offset.x())
                    .map(|(ix, _)| ix)
                    .last())
                .unwrap();
        }

        0
    }

    fn calculate_part2(&self) -> Self::Output { todo!() }
}

fn main() { execute_day::<Day>() }

#[cfg(test)]
mod tests {
    use advent_lib::day_test;

    use crate::{parse_commands, Command};

    day_test!( 22, example => 6032, 5031 );
    //day_test!( 22 => 197160, 0 );

    #[test]
    fn test_command_parsing() {
        let commands = parse_commands("10L10R1L1R3");
        assert_eq!(
            Ok(vec![
                Command::Forward { steps: 10 },
                Command::Left,
                Command::Forward { steps: 10 },
                Command::Right,
                Command::Forward { steps: 1 },
                Command::Left,
                Command::Forward { steps: 1 },
                Command::Right,
                Command::Forward { steps: 3 },
            ]),
            commands
        )
    }
}
