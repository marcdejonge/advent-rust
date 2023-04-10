use advent_lib::day::*;
use lazy_static::lazy_static;
use regex::Regex;
use std::str::FromStr;

struct Day {
    stack_lines: Vec<Vec<char>>,
    command_lines: Vec<Command>,
}

impl FromIterator<String> for Day {
    fn from_iter<T: IntoIterator<Item = String>>(iter: T) -> Self {
        let mut iter = iter.into_iter();
        let mut day = Day {
            stack_lines: parse_stacks(iter.by_ref().take_while(|line| line != "").collect()),
            command_lines: iter.filter_map(|line| line.parse::<Command>().ok()).collect(),
        };
        day.command_lines.reverse();
        day
    }
}

impl ExecutableDay for Day {
    type Output = String;

    fn calculate_part1(&self) -> Self::Output { self.calculate(true) }

    fn calculate_part2(&self) -> Self::Output { self.calculate(false) }
}

fn main() { execute_day::<Day>() }

fn parse_stacks(lines: Vec<String>) -> Vec<Vec<char>> {
    let stack_count = lines.last().expect("Could not find stacks").split("  ").count();
    let mut stacks = Vec::<_>::new();
    for ix in 0..stack_count {
        stacks.push(
            lines
                .iter()
                .filter_map(|line| line.chars().nth(ix * 4 + 1))
                .filter(|c| ('A'..='Z').contains(c))
                .collect(),
        );
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
        lazy_static! {
            static ref COMMAND_REGEX: Regex =
                Regex::new("move (\\d+) from (\\d+) to (\\d+)").unwrap();
        }
        let groups =
            COMMAND_REGEX.captures(line).ok_or(format!("Invalid command line: {}", line))?;
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

impl Day {
    fn calculate(&self, reversed: bool) -> String {
        (0..self.stack_lines.len())
            .map(|stack_ix| {
                let pos = self
                    .command_lines
                    .iter()
                    .fold(Position { stack_ix, char_ix: 0 }, |pos, command| {
                        pos.trace_back_command(command, reversed)
                    });
                self.stack_lines[pos.stack_ix][pos.char_ix]
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
