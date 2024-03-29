#![feature(test)]

use std::str::FromStr;

use advent_lib::day::*;

struct Day {
    dir_sizes: Vec<u32>,
}

impl ExecutableDay for Day {
    type Output = u32;

    fn from_lines<LINES: Iterator<Item = String>>(lines: LINES) -> Self {
        Day {
            dir_sizes: TraverseWithStack {
                iter: lines.filter_map(|line| line.parse::<Command>().ok()),
                stack: Vec::new(),
            }
            .collect(),
        }
    }

    fn calculate_part1(&self) -> Self::Output {
        self.dir_sizes.iter().filter(|&&size| size < 100000).sum()
    }

    fn calculate_part2(&self) -> Self::Output {
        let min_size = self.dir_sizes.last().unwrap_or(&0) - 40000000;
        self.dir_sizes
            .iter()
            .cloned()
            .filter(|&size| size >= min_size)
            .min()
            .expect("Could not find any")
    }
}

#[derive(Debug, PartialEq, Eq)]
enum Command {
    CdUp,
    CdDown,
    File(u32),
}

impl FromStr for Command {
    type Err = ();

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        if line == "$ cd .." {
            Ok(Command::CdUp)
        } else if line.starts_with("$ cd ") {
            Ok(Command::CdDown)
        } else if line.starts_with('$') || line.starts_with("dir ") {
            Err(())
        } else {
            let file_size = line.find(' ').map(|ix| &line[..ix]).ok_or(())?;
            file_size.parse::<u32>().map(Command::File).map_err(|_| ())
        }
    }
}

struct TraverseWithStack<I, S> {
    iter: I,
    stack: Vec<S>,
}

impl<I> TraverseWithStack<I, u32> {
    fn pop(&mut self) -> Option<u32> {
        let current = self.stack.pop()?;
        if let Some(parent) = self.stack.last_mut() {
            *parent += current;
        }
        Some(current)
    }
}

impl<I> Iterator for TraverseWithStack<I, u32>
where
    I: Iterator<Item = Command>,
{
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if let Some(ref command) = self.iter.next() {
                match command {
                    Command::CdUp => return self.pop(),
                    Command::CdDown => self.stack.push(0),
                    Command::File(file_size) => {
                        if let Some(current) = self.stack.last_mut() {
                            *current += file_size;
                        }
                    }
                }
            } else {
                return self.pop();
            }
        }
    }
}

fn main() { execute_day::<Day>() }

#[cfg(test)]
mod tests {
    use advent_lib::day_test;

    day_test!( 7, example => 95437, 24933642 );
    day_test!( 7 => 1086293, 366028 );
}
