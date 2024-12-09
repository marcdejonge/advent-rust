#![feature(test)]

use advent_lib::day::*;
use advent_lib::iter_utils::IteratorUtils;

struct Day {
    input: Vec<u8>,
}

mod contiguous_memory {
    pub(crate) struct Memory {
        memory: Vec<u32>,
    }

    impl Memory {
        const EMPTY: u32 = u32::MAX;

        pub(crate) fn defragment_memory(&mut self) {
            let mut left_empty_ix = 0;
            let mut right_ix = self.memory.len() - 1;
            while left_empty_ix < right_ix {
                while self.memory[right_ix] == Self::EMPTY {
                    right_ix -= 1;
                }
                while self.memory[left_empty_ix] != Self::EMPTY {
                    left_empty_ix += 1;
                }
                if left_empty_ix < right_ix {
                    self.memory.swap(left_empty_ix, right_ix);
                    right_ix -= 1;
                    left_empty_ix += 1;
                }
            }
        }

        pub(crate) fn checksum_memory(&self) -> u128 {
            self.memory
                .iter()
                .take_while(|&&file_ix| file_ix != Self::EMPTY)
                .enumerate()
                .map(|(ix, &file_ix)| file_ix as u128 * ix as u128)
                .sum()
        }
    }

    impl From<&super::Day> for Memory {
        fn from(day: &super::Day) -> Self {
            let mut memory = Vec::new();

            for (location, input) in day.input.chunks(2).enumerate() {
                if input.len() >= 1 {
                    for _ in 0..input[0] {
                        memory.push(location as u32);
                    }
                }
                if input.len() == 2 {
                    for _ in 0..input[1] {
                        memory.push(Memory::EMPTY);
                    }
                }
            }

            Memory { memory }
        }
    }
}

mod sparse_memory {
    use std::cmp::max;

    pub(crate) struct Memory {
        files: Vec<File>,
        free_space: Vec<Space>,
    }

    struct File {
        file_ix: u32,
        size: u8,
        location: u32,
    }

    struct Space {
        size: u8,
        location: u32,
    }

    impl From<&super::Day> for Memory {
        fn from(day: &super::Day) -> Self {
            let mut files = Vec::new();
            let mut free_space = Vec::new();
            let mut file_ix = 0;
            let mut location = 0;
            let mut empty = false;

            for &size in &day.input {
                if empty {
                    free_space.push(Space { size, location });
                    location += size as u32;
                    empty = false;
                } else {
                    files.push(File { file_ix, size, location });
                    location += size as u32;
                    file_ix += 1;
                    empty = true;
                }
            }

            Memory { files, free_space }
        }
    }

    impl Memory {
        pub(crate) fn defragment_files(&mut self) {
            let mut first_free_space = [0usize; 10];

            for file in self.files.iter_mut().rev() {
                let size_ix = file.size as usize;
                for space_ix in first_free_space[size_ix]..self.free_space.len() {
                    let space = &mut self.free_space[space_ix];

                    if space.location > file.location {
                        break;
                    } else if space.size >= file.size {
                        file.location = space.location;
                        space.size -= file.size;
                        space.location += file.size as u32;
                        first_free_space[size_ix] = max(first_free_space[size_ix], space_ix);
                        break;
                    }
                }
            }
        }

        pub(crate) fn checksum_files(&self) -> u128 {
            self.files
                .iter()
                .flat_map(|file| {
                    (0..file.size).map(|byte_ix| {
                        file.file_ix as u128 * (file.location + byte_ix as u32) as u128
                    })
                })
                .sum()
        }
    }
}

impl ExecutableDay for Day {
    type Output = u128;

    fn from_lines<LINES: Iterator<Item = String>>(lines: LINES) -> Self {
        Day { input: lines.single().unwrap().bytes().map(|c| c - b'0').collect() }
    }
    fn calculate_part1(&self) -> Self::Output {
        let mut memory: contiguous_memory::Memory = self.into();
        memory.defragment_memory();
        memory.checksum_memory()
    }
    fn calculate_part2(&self) -> Self::Output {
        let mut memory: sparse_memory::Memory = self.into();
        memory.defragment_files();
        memory.checksum_files()
    }
}

fn main() { execute_day::<Day>() }

#[cfg(test)]
mod tests {
    use advent_lib::day_test;

    day_test!( 9, example1 => 1928, 2858 );
    day_test!( 9 => 6258319840548, 6286182965311);
}
