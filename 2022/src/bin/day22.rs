#![feature(test)]

use advent_lib::day::{execute_day, ExecutableDay};
use advent_lib::direction::Direction;
use advent_lib::geometry::{point2, vector2, Point, Vector};
use advent_lib::grid::Grid;
use advent_macros::FromRepr;
use fxhash::{FxBuildHasher, FxHashMap};
use std::cmp::min;
use std::ops::Neg;
use Direction::*;

use crate::FieldType::*;

#[derive(Debug, PartialEq)]
enum Command {
    Forward(u32),
    Left,
    Right,
}

struct Person {
    position: Point<2, i32>,
    direction: Direction,
}

impl Person {
    fn step(&self) -> Point<2, i32> { self.position + self.direction.as_vec() }

    fn score(&self) -> i32 {
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
            commands.push(Command::Forward(number));
            number = 0;
            match b {
                b'L' => commands.push(Command::Left),
                b'R' => commands.push(Command::Right),
                _ => return Err(format!("Unexpected character {b}")),
            }
        }
    }

    if number > 0 {
        commands.push(Command::Forward(number))
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

#[derive(Debug)]
struct Day {
    commands: Vec<Command>,
    block_size: usize,
    blocks: FxHashMap<Vector<2, i32>, Grid<FieldType>>,
    start_offset: Vector<2, i32>,
}

impl Day {
    fn next_block_2d(
        &self,
        current_offset: &Vector<2, i32>,
        direction: &Direction,
    ) -> (Vector<2, i32>, &Grid<FieldType>) {
        let step = direction.as_vec() * self.block_size as i32;
        let mut next_offset = *current_offset;
        let modulus = 4 * self.block_size as i32;

        for _ in 0..5 {
            next_offset = next_offset + step;
            next_offset = vector2(
                (next_offset.x() + modulus) % modulus,
                (next_offset.y() + modulus) % modulus,
            );
            if let Some(block) = self.blocks.get(&next_offset) {
                return (next_offset, block);
            }
        }

        panic!("No block found")
    }
}

impl ExecutableDay for Day {
    type Output = i32;

    fn from_lines<LINES: Iterator<Item = String>>(mut lines: LINES) -> Self {
        let mut field_lines: Vec<String> =
            lines.by_ref().take_while(|line| !line.is_empty()).collect();
        let width = field_lines.iter().map(|line| line.len()).max().unwrap_or(0);
        field_lines.iter_mut().for_each(|line| *line = format!("{:width$}", line));

        let field = Grid::from(field_lines.into_iter());
        let commands = parse_commands(lines.next().unwrap().as_str()).unwrap();

        let mut blocks = FxHashMap::with_capacity_and_hasher(10, FxBuildHasher::default());
        let block_size = (min(field.x_range().end, field.y_range().end) / 3) as usize;

        for y in field.y_range().step_by(block_size) {
            for x in field.x_range().step_by(block_size) {
                if field[point2(x, y)] != Outside {
                    blocks.insert(
                        vector2(x, y),
                        field.sub_grid(x..(x + block_size as i32), y..(y + block_size as i32)),
                    );
                }
            }
        }

        let start_block = blocks.keys().find(|offset| offset.y() == 0).unwrap().clone();

        Day { commands, block_size, blocks, start_offset: start_block }
    }

    fn calculate_part1(&self) -> Self::Output {
        let mut current_offset = self.start_offset;
        let mut current_block = self.blocks.get(&current_offset).unwrap();
        let mut person = Person { position: point2(0, 0), direction: East };

        for command in &self.commands {
            match command {
                Command::Forward(steps) => {
                    for _ in 0..*steps {
                        let mut next_position = person.step();
                        match current_block.get(next_position) {
                            Some(Empty) => person.position = next_position,
                            Some(Wall) => break,
                            Some(Outside) => panic!("Outside a block should not be possible"),
                            None => {
                                let (next_offset, next_block) =
                                    self.next_block_2d(&current_offset, &person.direction);
                                next_position = next_position
                                    + (person.direction.neg().as_vec() * (self.block_size as i32));

                                if next_block.get(next_position) == Some(&Wall) {
                                    break;
                                }

                                current_offset = next_offset;
                                current_block = next_block;
                                person.position = next_position;
                            }
                        }
                    }
                }
                Command::Left => person.direction = person.direction.turn_left(),
                Command::Right => person.direction = person.direction.turn_right(),
            }
        }

        person.position = person.position + current_offset;
        person.score()
    }

    fn calculate_part2(&self) -> Self::Output { todo!() }
}

fn main() { execute_day::<Day>() }

#[cfg(test)]
mod tests {
    use advent_lib::day_test;

    use crate::{parse_commands, Command};

    day_test!( 22, example => 6032, 5031 );
    day_test!( 22 => 197160, 0 );

    #[test]
    fn test_command_parsing() {
        let commands = parse_commands("10L10R1L1R3");
        assert_eq!(
            Ok(vec![
                Command::Forward(10),
                Command::Left,
                Command::Forward(10),
                Command::Right,
                Command::Forward(1),
                Command::Left,
                Command::Forward(1),
                Command::Right,
                Command::Forward(3),
            ]),
            commands
        )
    }
}
