#![feature(test)]

use advent_lib::day::*;
use advent_lib::iter_utils::IteratorUtils;

struct Day {
    input: Vec<u8>,
}

const EMPTY: u32 = u32::MAX;

impl Day {
    fn generate_memory(&self) -> Vec<u32> {
        let mut memory = Vec::new();
        let mut location = 0;
        let mut empty = false;

        for &size in &self.input {
            for _ in 0..size {
                if empty {
                    memory.push(EMPTY);
                } else {
                    memory.push(location);
                }
            }

            if empty {
                location += 1;
                empty = false;
            } else {
                empty = true;
            }
        }

        memory
    }

    fn defragment_memory(memory: &mut Vec<u32>) {
        let mut left_empty_ix = 0;
        let mut right_ix = memory.len() - 1;
        while left_empty_ix < right_ix {
            while memory[right_ix] == EMPTY {
                right_ix -= 1;
            }
            while memory[left_empty_ix] != EMPTY {
                left_empty_ix += 1;
            }
            if left_empty_ix < right_ix {
                memory.swap(left_empty_ix, right_ix);
            }
        }
    }

    fn checksum_memory(memory: &mut Vec<u32>) -> u64 {
        memory
            .iter()
            .take_while(|&&file_ix| file_ix != EMPTY)
            .enumerate()
            .map(|(ix, &file_ix)| file_ix as u64 * ix as u64)
            .sum()
    }

    fn generate_files(&self) -> (Vec<File>, Vec<Space>) {
        let mut files = Vec::new();
        let mut free_space = Vec::new();
        let mut file_ix = 0;
        let mut location = 0;
        let mut empty = false;

        for &size in &self.input {
            let size = size as u64;
            if empty {
                free_space.push(Space { size, location });
                location += size;
                empty = false;
            } else {
                files.push(File { file_ix, size, location });
                location += size;
                file_ix += 1;
                empty = true;
            }
        }

        (files, free_space)
    }

    fn defragment_files(files: &mut Vec<File>, spaces: &mut Vec<Space>) {
        for file in files.iter_mut().rev() {
            let mut space_ix = 0;
            while space_ix < spaces.len() - 1 {
                let space = &mut spaces[space_ix];
                if space.location > file.location {
                    break;
                }

                if space.size >= file.size {
                    file.location = space.location;
                    space.size -= file.size;
                    if space.size == 0 {
                        spaces.remove(space_ix);
                    } else {
                        space.location += file.size;
                    }
                    break;
                }

                // Compaction of free space
                if spaces[space_ix].location + spaces[space_ix].size
                    == spaces[space_ix + 1].location
                {
                    spaces[space_ix].size += spaces[space_ix + 1].size;
                    spaces.remove(space_ix + 1);
                } else {
                    space_ix += 1; // Only move to the next space if this one wasn't expanded
                }
            }
        }
    }

    fn checksum_files(files: &Vec<File>) -> u64 {
        files
            .iter()
            .flat_map(|file| (0..file.size).map(|byte_ix| file.file_ix * (file.location + byte_ix)))
            .sum()
    }
}

struct File {
    file_ix: u64,
    size: u64,
    location: u64,
}

struct Space {
    size: u64,
    location: u64,
}

impl ExecutableDay for Day {
    type Output = u64;

    fn from_lines<LINES: Iterator<Item = String>>(lines: LINES) -> Self {
        Day { input: lines.single().unwrap().bytes().map(|c| c - b'0').collect() }
    }
    fn calculate_part1(&self) -> Self::Output {
        let mut memory = self.generate_memory();
        Self::defragment_memory(&mut memory);
        Self::checksum_memory(&mut memory)
    }
    fn calculate_part2(&self) -> Self::Output {
        let (mut files, mut spaces) = self.generate_files();
        Self::defragment_files(&mut files, &mut spaces);
        Self::checksum_files(&files)
    }
}

fn main() { execute_day::<Day>() }

#[cfg(test)]
mod tests {
    use advent_lib::day_test;

    day_test!( 9, example1 => 1928, 2858 );
    day_test!( 9 => 6258319840548, 6286182965311);
}
