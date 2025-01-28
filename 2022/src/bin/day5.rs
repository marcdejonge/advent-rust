#![feature(test)]

use advent_lib::day_main;
use advent_lib::parsing::{
    double_line_ending, in_brackets, separated_lines1, single_match, single_space,
};
use nom_parse_macros::parse_from;

#[derive(Debug)]
#[parse_from(separated_pair(
    map(
        separated_list1(
            line_ending,
            separated_list1(
                single_space(),
                alt(
                    in_brackets(single_match(AsChar::is_alpha)),
                    map("   ", |_| b' '),
                    delimited(single_space(), single_match(AsChar::is_dec_digit), opt(single_space())),
                )
            )
        ),
        parse_stacks,
    ),
    double_line_ending,
    separated_lines1(),
))]
struct Input {
    stack_lines: Vec<Vec<u8>>,
    command_lines: Vec<Command>,
}

fn calculate_part1(input: &Input) -> String { input.calculate(true) }

fn calculate_part2(input: &Input) -> String { input.calculate(false) }

day_main!();

fn parse_stacks(lines: Vec<Vec<u8>>) -> Vec<Vec<u8>> {
    let stack_count = lines.last().expect("Could not find stacks").len();
    let mut stacks = Vec::<_>::new();
    for ix in 0..stack_count {
        stacks.push(
            lines
                .iter()
                .filter_map(|line| line.get(ix).copied())
                .filter(u8::is_ascii_uppercase)
                .collect(),
        );
    }
    stacks
}

#[derive(Debug, Clone)]
#[parse_from(match "move {} from {} to {}")]
struct Command {
    count: u32,
    from_stack_ix: u32,
    to_stack_ix: u32,
}

#[derive(Debug, Clone, Copy)]
struct Position {
    stack_ix: u32,
    char_ix: u32,
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

impl Input {
    fn calculate(&self, reversed: bool) -> String {
        (0..self.stack_lines.len())
            .map(|stack_ix| {
                let pos = self.command_lines.iter().rev().fold(
                    Position { stack_ix: stack_ix as u32, char_ix: 0 },
                    |pos, command| {
                        let mut command = command.clone();
                        command.from_stack_ix -= 1;
                        command.to_stack_ix -= 1;
                        pos.trace_back_command(&command, reversed)
                    },
                );
                self.stack_lines[pos.stack_ix as usize][pos.char_ix as usize] as char
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use advent_lib::day_test;

    day_test!( 5, example => "CMZ".to_owned(), "MCD".to_owned());
    day_test!( 5 => "QPJPLMNNR".to_owned(), "BQDNWJPVJ".to_owned());
}
