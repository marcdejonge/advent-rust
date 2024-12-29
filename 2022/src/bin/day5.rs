#![feature(test)]

use advent_lib::day_main;
use advent_macros::parsable;

#[derive(Debug)]
#[parsable(separated_pair(
    map(
        separated_list1(
            line_ending,
            separated_list1(
                single_space(),
                alt((
                    in_brackets(single_match(is_alphabetic)),
                    map(tag(b"   "), |_| b' '),
                    delimited(single_space(), single_match(is_digit), opt(single_space())),
                ))
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

#[derive(Debug)]
#[parsable(tuple((
    preceded(tag(b"move "), usize::parser()),
    map(preceded(tag(b" from "), usize::parser()), |nr| nr - 1),
    map(preceded(tag(b" to "), usize::parser()), |nr| nr - 1),
)))]
struct Command {
    count: usize,
    from_stack_ix: usize,
    to_stack_ix: usize,
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

impl Input {
    fn calculate(&self, reversed: bool) -> String {
        (0..self.stack_lines.len())
            .map(|stack_ix| {
                let pos = self
                    .command_lines
                    .iter()
                    .rev()
                    .fold(Position { stack_ix, char_ix: 0 }, |pos, command| {
                        pos.trace_back_command(command, reversed)
                    });
                self.stack_lines[pos.stack_ix][pos.char_ix] as char
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
