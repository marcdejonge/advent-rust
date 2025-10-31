#![feature(test)]

use advent_lib::direction::Direction;
use advent_lib::geometry::{point2, vector2, Point, Vector};
use advent_lib::grid::{uneven_grid_parser, Grid};
use advent_lib::parsing::double_line_ending;
use advent_lib::*;
use advent_macros::FromRepr;
use fxhash::FxHashMap;
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

#[derive(Debug, Copy, Clone)]
struct Person {
    pos: Point<2, i32>,
    dir: Direction,
}

impl Person {
    fn step(&self) -> Person { Person { dir: self.dir, pos: self.pos + self.dir } }

    fn score(&self) -> i32 {
        (self.pos.x() + 1) * 4 + (self.pos.y() + 1) * 1000 + i32::from(self.dir)
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
struct Block {
    grid: Grid<FieldType>,
    offset: Vector<2, i32>,
}

#[derive(Debug)]
#[parse_from(map({}, |input: GridAndCommands| {
    let mut blocks = vec![];
    let block_size = min(input.grid.x_range().end, input.grid.y_range().end) / 3;

    for y in input.grid.y_range().step_by(block_size as usize) {
        for x in input.grid.x_range().step_by(block_size as usize) {
            if input.grid.get(point2(x, y)).cloned().unwrap_or_default() != Outside {
                blocks.push(Block {
                    grid: input.grid.sub_grid(x..(x + block_size), y..(y + block_size)),
                    offset: vector2(x, y),
                });
            }
        }
    }

    let blocks = blocks.try_into().expect("Expected 6 blocks as input");

    (
        input.commands,
        block_size,
        calc_block_jumps_2d(&blocks),
        calc_block_jumps_3d(&blocks),
        blocks,
    )
}))]
struct Input {
    commands: Vec<Command>,
    block_size: i32,
    block_jump_2d: BlockJumps,
    block_jump_3d: BlockJumps,
    blocks: [Block; 6],
}

#[derive(Debug, Copy, Clone)]
enum Turn {
    None,
    Left,
    Right,
    Around,
}

impl Turn {
    fn apply(&self, person: &Person, block_size: i32) -> Person {
        let position = person.pos + (person.dir.neg().as_vec() * block_size);
        match self {
            Self::None => Person { dir: person.dir, pos: position },
            Self::Left => Person {
                dir: person.dir.turn_left(),
                pos: point2(position.y(), block_size - position.x() - 1),
            },
            Self::Right => Person {
                dir: person.dir.turn_right(),
                pos: point2(block_size - position.y() - 1, position.x()),
            },
            Self::Around => Person {
                dir: -person.dir,
                pos: point2(
                    block_size - position.x() - 1,
                    (block_size) - position.y() - 1,
                ),
            },
        }
    }
}

type BlockJumps = FxHashMap<(usize, Direction), (usize, Turn)>;

fn calc_block_jumps_2d(blocks: &[Block; 6]) -> BlockJumps {
    let mut jumps = FxHashMap::default();
    let block_size = blocks[0].grid.width();

    for start_ix in 0..6 {
        let start_offset = blocks[start_ix].offset;
        for direction in Direction::ALL {
            let step = direction.as_vec() * block_size;
            let modulus = 4 * block_size;
            let mut next_offset = start_offset;

            for _ in 1..=4 {
                next_offset = (next_offset + step + vector2(modulus, modulus)) % modulus;
                if let Some((next_ix, _)) =
                    blocks.iter().enumerate().find(|(_, block)| block.offset == next_offset)
                {
                    jumps.insert((start_ix, direction), (next_ix, Turn::None));
                    break;
                }
            }
        }
    }

    jumps
}

fn calc_block_jumps_3d(blocks: &[Block; 6]) -> BlockJumps {
    let mut jumps = FxHashMap::default();
    let block_size = blocks[0].grid.width();

    if block_size == 4 {
        jumps.insert((0, North), (1, Turn::Around));
        jumps.insert((0, East), (5, Turn::Around));
        jumps.insert((0, South), (3, Turn::None));
        jumps.insert((0, West), (2, Turn::Left));

        jumps.insert((1, North), (0, Turn::Around));
        jumps.insert((1, East), (2, Turn::None));
        jumps.insert((1, South), (4, Turn::Around));
        jumps.insert((1, West), (5, Turn::Left));

        jumps.insert((2, North), (0, Turn::Right));
        jumps.insert((2, East), (3, Turn::None));
        jumps.insert((2, South), (4, Turn::Left));
        jumps.insert((2, West), (1, Turn::None));

        jumps.insert((3, North), (0, Turn::None));
        jumps.insert((3, East), (5, Turn::Right));
        jumps.insert((3, South), (4, Turn::None));
        jumps.insert((3, West), (2, Turn::None));

        jumps.insert((4, North), (3, Turn::None));
        jumps.insert((4, East), (5, Turn::None));
        jumps.insert((4, South), (1, Turn::Around));
        jumps.insert((4, West), (2, Turn::Right));

        jumps.insert((5, North), (3, Turn::Left));
        jumps.insert((5, East), (0, Turn::Around));
        jumps.insert((5, South), (1, Turn::Right));
        jumps.insert((5, West), (4, Turn::None));
    } else {
        jumps.insert((0, North), (5, Turn::Right));
        jumps.insert((0, East), (1, Turn::None));
        jumps.insert((0, South), (2, Turn::None));
        jumps.insert((0, West), (3, Turn::Around));

        jumps.insert((1, North), (5, Turn::None));
        jumps.insert((1, East), (4, Turn::Around));
        jumps.insert((1, South), (2, Turn::Right));
        jumps.insert((1, West), (0, Turn::None));

        jumps.insert((2, North), (0, Turn::None));
        jumps.insert((2, East), (1, Turn::Left));
        jumps.insert((2, South), (4, Turn::None));
        jumps.insert((2, West), (3, Turn::Left));

        jumps.insert((3, North), (2, Turn::Right));
        jumps.insert((3, East), (4, Turn::None));
        jumps.insert((3, South), (5, Turn::None));
        jumps.insert((3, West), (0, Turn::Around));

        jumps.insert((4, North), (2, Turn::None));
        jumps.insert((4, East), (1, Turn::Around));
        jumps.insert((4, South), (5, Turn::Right));
        jumps.insert((4, West), (3, Turn::None));

        jumps.insert((5, North), (3, Turn::None));
        jumps.insert((5, East), (4, Turn::Left));
        jumps.insert((5, South), (1, Turn::None));
        jumps.insert((5, West), (0, Turn::Left));
    }

    jumps
}

fn handle_command(
    input: &Input,
    block_jump: &BlockJumps,
    command: &Command,
    mut index: usize,
    mut person: Person,
) -> (usize, Person) {
    match command {
        Command::Forward(steps) => {
            for _ in 0..*steps {
                let mut next_person = person.step();
                match input.blocks[index].grid.get(next_person.pos) {
                    Some(Empty) => person = next_person,
                    Some(Wall) => break,
                    Some(Outside) => panic!("Outside a block should not be possible"),
                    None => {
                        let (next_ix, turn) = block_jump[&(index, person.dir)];
                        next_person = turn.apply(&next_person, input.block_size);

                        if input.blocks[next_ix].grid.get(next_person.pos) == Some(&Wall) {
                            break;
                        }

                        (index, person) = (next_ix, next_person);
                    }
                }
            }
        }
        Command::Left => person.dir = person.dir.turn_left(),
        Command::Right => person.dir = person.dir.turn_right(),
    }

    (index, person)
}

fn calculate(input: &Input, block_jump: &BlockJumps) -> i32 {
    let mut index = 0;
    let mut person = Person { pos: point2(0, 0), dir: East };

    for command in &input.commands {
        (index, person) = handle_command(input, block_jump, command, index, person);
    }

    person.pos += input.blocks[index].offset;
    person.score()
}

fn calculate_part1(input: &Input) -> i32 { calculate(input, &input.block_jump_2d) }

fn calculate_part2(input: &Input) -> i32 { calculate(input, &input.block_jump_3d) }

day_main!(Input);
day_test!( 22, example => 6032, 5031 );
day_test!( 22 => 197160, 145065 );

#[cfg(test)]
mod tests {
    use crate::Command;
    use nom::multi::many1;
    use nom::IResult;
    use nom::Parser;
    use nom_parse_trait::ParseFrom;

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
