use std::str::FromStr;

use lazy_static::lazy_static;
use regex::Regex;

crate::day!(5, (Vec<Vec<char>>, Vec<Command>), String {
    parse_input(input) {
        let mut lines = input.lines();
        let stack_lines = lines.by_ref().take_while(|&line| line != "").collect();
        let command_lines = lines.filter_map(|line| line.parse::<Commad>().ok()).rev().collect();

        (parse_stacks(stack_lines), command_lines)
    }

    calculate_part1(input) { calculate(input, true) }
    calculate_part2(input) { calculate(input, false) }

    example_input(
        "    [D]
[N] [C]
[Z] [M] [P]
 1   2   3

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2"
        => "CMZ", "MCD"
    )
});

fn parse_stacks(lines: Vec<&str>) -> Vec<Vec<char>> {
    let stack_count = lines.last().expect("Could not find stacks").split("  ").count();
    let mut stacks = Vec::<_>::new();
    for ix in 0..stack_count {
        stacks.push(lines.iter()
            .filter_map(|&line| line.chars().nth(ix * 4 + 1))
            .filter(|c| ('A'..='Z').contains(c))
            .collect());
    }
    stacks
}

#[derive(Debug)]
struct Command {
    count: usize,
    from_stack_ix: usize,
    to_stack_ix: usize,
}

impl FromStr for Command {
    type Err = String;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        lazy_static! { static ref COMMAND_REGEX: Regex = Regex::new("move (\\d+) from (\\d+) to (\\d+)").unwrap(); }
        let groups = COMMAND_REGEX.captures(line).ok_or(format!("Invalid command line: {}", line))?;
        Ok(Command {
            count: groups[1].parse().unwrap(),
            from_stack_ix: groups[2].parse::<usize>().unwrap() - 1,
            to_stack_ix: groups[3].parse::<usize>().unwrap() - 1,
        })
    }
}

#[derive(Debug, Clone, Copy)]
struct Position {
    stack_ix: usize,
    char_ix: usize,
}

impl Position {
    #[inline]
    fn trace_back_command(mut self, command: &Command, reversed: bool) -> Position {
        if command.from_stack_ix == self.stack_ix {
            self.char_ix += command.count;
        } else if command.to_stack_ix == self.stack_ix {
            if self.char_ix >= command.count {
                self.char_ix -= command.count;
            } else {
                self.stack_ix = command.from_stack_ix;
                if reversed {
                    self.char_ix = command.count - (self.char_ix + 1)
                }
            }
        }
        self
    }
}

#[inline]
fn calculate(input: &(Vec<Vec<char>>, Vec<Command>), reversed: bool) -> String {
    let (stacks, commands) = input;
    (0..stacks.len()).map(|stack_ix| {
        let pos = commands.iter().fold(Position { stack_ix, char_ix: 0 }, |pos, command| {
            pos.trace_back_command(command, reversed)
        });
        stacks[pos.stack_ix][pos.char_ix]
    }).collect()
}
