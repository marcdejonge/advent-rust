#![feature(test)]

use advent_lib::day_main;
use advent_lib::direction::Direction;
use advent_lib::geometry::{point2, vector2, Point, Vector};
use advent_lib::grid::{uneven_grid_parser, Grid};
use advent_lib::parsing::double_line_ending;
use advent_macros::FromRepr;
use fxhash::{FxBuildHasher, FxHashMap};
use nom_parse_macros::parse_from;
use std::cmp::min;
use std::ops::Neg;
use Direction::*;

use crate::FieldType::*;

#[derive(Debug, PartialEq)]
#[parse_from]
enum Command {
    #[format(())]
    Forward(u32),
    #[format('L')]
    Left,
    #[format('R')]
    Right,
}

struct Person {
    position: Point<2, i32>,
    direction: Direction,
}

impl Person {
    fn step(&self) -> Point<2, i32> { self.position + self.direction }

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

#[repr(u8)]
#[derive(FromRepr, Default, Clone, Copy, PartialEq)]
enum FieldType {
    #[default]
    Outside = b' ',
    Empty = b'.',
    Wall = b'#',
}

#[parse_from(separated_pair(uneven_grid_parser, double_line_ending, many1(Command::parse)))]
struct GridAndCommands {
    grid: Grid<FieldType>,
    commands: Vec<Command>,
}

#[derive(Debug)]
struct Input {
    commands: Vec<Command>,
    block_size: usize,
    blocks: FxHashMap<Vector<2, i32>, Grid<FieldType>>,
    start_offset: Vector<2, i32>,
}

fn preprocess(input: GridAndCommands) -> Input {
    let mut blocks = FxHashMap::with_capacity_and_hasher(10, FxBuildHasher::default());
    let block_size = (min(input.grid.x_range().end, input.grid.y_range().end) / 3) as usize;

    for y in input.grid.y_range().step_by(block_size) {
        for x in input.grid.x_range().step_by(block_size) {
            if input.grid.get(point2(x, y)).cloned().unwrap_or_default() != Outside {
                blocks.insert(
                    vector2(x, y),
                    input.grid.sub_grid(x..(x + block_size as i32), y..(y + block_size as i32)),
                );
            }
        }
    }

    let start_block = *blocks.keys().find(|offset| offset.y() == 0).unwrap();

    Input { commands: input.commands, block_size, blocks, start_offset: start_block }
}

impl Input {
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

fn calculate_part1(input: &Input) -> i32 {
    let mut current_offset = input.start_offset;
    let mut current_block = input.blocks.get(&current_offset).unwrap();
    let mut person = Person { position: point2(0, 0), direction: East };

    for command in &input.commands {
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
                                input.next_block_2d(&current_offset, &person.direction);
                            next_position = next_position
                                + (person.direction.neg().as_vec() * (input.block_size as i32));

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

fn calculate_part2(_input: &Input) -> i32 { todo!() }

day_main!( preprocess => calculate_part1, calculate_part2 );

#[cfg(test)]
mod tests {
    use crate::Command;
    use advent_lib::day_test;
    use nom::multi::many1;
    use nom::IResult;
    use nom::Parser;
    use nom_parse_trait::ParseFrom;

    day_test!( 22, example => 6032/*, 5031 */ ; preprocess );
    day_test!( 22 => 197160 ; preprocess );

    #[test]
    fn test_command_parsing() {
        let commands: IResult<_, _> = many1(Command::parse).parse("10L10R1L1R3");
        let commands = commands.unwrap().1;
        assert_eq!(
            vec![
                Command::Forward(10),
                Command::Left,
                Command::Forward(10),
                Command::Right,
                Command::Forward(1),
                Command::Left,
                Command::Forward(1),
                Command::Right,
                Command::Forward(3),
            ],
            commands
        )
    }
}
