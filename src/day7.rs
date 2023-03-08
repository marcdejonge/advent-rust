use std::str::FromStr;

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
        } else if line.starts_with("$") || line.starts_with("dir ") {
            Err(())
        } else {
            let file_size = line.find(' ').map(|ix| &line[..ix]).ok_or(())?;
            file_size
                .parse::<u32>()
                .map(|size| Command::File(size))
                .map_err(|_| ())
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
        return Some(current);
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
