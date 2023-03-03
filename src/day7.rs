use std::str::FromStr;

use lazy_static::lazy_static;
use regex::Regex;

crate::day!(7, Vec<u32>, u32 {
    parse_input(input) {
        TraverseWithStack { iter: input.lines().filter_map(|line| line.parse::<Command>().ok()), stack: Vec::new() }.collect()
    }

    calculate_part1(input) {
        input.iter().filter(|&&size| size < 100000).sum()
    }

    calculate_part2(input) {
        let min_size = input.last().unwrap_or(&0) - 40000000;
        input.iter().filter(|&&size| size >= min_size).min().expect("Could not find any").clone()
    }

    test example(include_str!("example_input/day7.txt") => 95437, 24933642)
});

#[derive(Debug, PartialEq, Eq)]
enum Command {
    Cd(String),
    File(u32),
}

impl FromStr for Command {
    type Err = String;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        lazy_static!( static ref LINE_PARSER: Regex = Regex::new("(\\S+) (\\S+)( (\\S+))?").unwrap(); );
        let captures = LINE_PARSER.captures(line).ok_or(format!("Cannot parse line: {}", line))?;
        let name = &captures[2];
        match &captures[1] {
            "$" => if name == "cd" {
                Ok(Command::Cd(captures[4].to_string()))
            } else {
                Err(format!("Other command than cd"))
            },
            "dir" => Err(format!("directories should be ignored")),
            _ => captures[1].parse::<u32>().map(|size| Command::File(size)).map_err(|_| format!("Invalid line: {}", line)),
        }
    }
}

pub struct TraverseWithStack<I, S> {
    pub iter: I,
    pub stack: Vec<S>,
}

impl<I> TraverseWithStack<I, u32> {
    fn pop(&mut self) -> Option<u32> {
        let current = self.stack.pop()?;
        if let Some(parent) = self.stack.last_mut() {
            *parent += current;
        }
        return Some(current);
    }

    fn push_dir(&mut self) {
        self.stack.push(0);
    }
}

impl<I> Iterator for TraverseWithStack<I, u32> where I: Iterator<Item=Command> {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if let Some(ref command) = self.iter.next() {
                match command {
                    Command::Cd(dir_name) => match dir_name.as_str() {
                        ".." => return self.pop(),
                        _ => self.push_dir(),
                    }
                    Command::File(file_size) => if let Some(current) = self.stack.last_mut() {
                        *current += file_size;
                    }
                }
            } else {
                return self.pop();
            }
        }
    }
}
