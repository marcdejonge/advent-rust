#![feature(test)]

use advent_lib::day_main;
use nom_parse_macros::parse_from;
use std::str::FromStr;

fn precompute(commands: Vec<Command>) -> Vec<u32> {
    TraverseWithStack { iter: commands.iter(), stack: vec![0] }.collect()
}

fn calculate_part1(dir_sizes: &Vec<u32>) -> u32 {
    dir_sizes.iter().filter(|&&size| size < 100000).sum()
}

fn calculate_part2(dir_sizes: &Vec<u32>) -> u32 {
    let min_size = dir_sizes.last().unwrap_or(&0) - 40000000;
    dir_sizes
        .iter()
        .cloned()
        .filter(|&size| size >= min_size)
        .min()
        .expect("Could not find any")
}

#[derive(Debug, PartialEq, Eq)]
#[parse_from]
enum Command {
    #[format("$ cd ..")]
    CdUp,
    #[format(preceded("$ cd ", not_line_ending))]
    CdDown,
    #[format(terminated(u32, not_line_ending))]
    File(u32),
    #[format(terminated("dir ", not_line_ending))]
    Dir,
    #[format("$ ls")]
    Ls,
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

impl<'a, I> Iterator for TraverseWithStack<I, u32>
where
    I: Iterator<Item = &'a Command>,
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
                    _ => {}
                }
            } else {
                return self.pop();
            }
        }
    }
}

day_main!( precompute => calculate_part1, calculate_part2 );

#[cfg(test)]
mod tests {
    use advent_lib::day_test;

    day_test!( 7, example => 95437, 24933642 ; precompute );
    day_test!( 7 => 1086293, 366028 ; precompute );
}
