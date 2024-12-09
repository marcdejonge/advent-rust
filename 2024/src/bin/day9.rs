#![feature(test)]

use advent_lib::day::*;
use advent_lib::iter_utils::IteratorUtils;
use std::cmp::max;

#[derive(Clone)]
struct Day {
    files: Vec<File>,
    free_space: Vec<Space>,
}

#[derive(Clone, Debug)]
struct File {
    file_ix: u32,
    size: u8,
    location: u32,
}

#[derive(Clone, Debug)]
struct Space {
    size: u8,
    location: u32,
}

impl Day {
    fn defragment_fractional_files(&mut self) {
        let mut space_ix = 0;
        let mut file_ix = self.files.len() - 1;
        loop {
            let space = self.free_space.get_mut(space_ix).unwrap();
            let last_file = self.files.get_mut(file_ix).unwrap();
            if last_file.location < space.location {
                break;
            }

            if space.size >= last_file.size {
                last_file.location = space.location;
                space.size -= last_file.size;
                space.location += last_file.size as u32;
                file_ix -= 1;
                if space.size == 0 {
                    space_ix += 1;
                }
            } else {
                let size = space.size;
                last_file.size -= size;
                space.size = 0;
                let file_ix = last_file.file_ix;
                self.files.push(File { file_ix, size, location: space.location });
                space_ix += 1;
            }
        }
    }

    fn defragment_whole_files(&mut self) {
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

    fn checksum(&self) -> u128 {
        self.files
            .iter()
            .flat_map(|file| {
                (0..file.size)
                    .map(|byte_ix| file.file_ix as u128 * (file.location + byte_ix as u32) as u128)
            })
            .sum()
    }
}

impl ExecutableDay for Day {
    type Output = u128;

    fn from_lines<LINES: Iterator<Item = String>>(lines: LINES) -> Self {
        let mut files = Vec::new();
        let mut free_space = Vec::new();
        let mut file_ix = 0;
        let mut location = 0;
        let mut empty = false;

        for size in lines.single().unwrap().bytes().map(|c| c - b'0') {
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

        Day { files, free_space }
    }
    fn calculate_part1(&self) -> Self::Output {
        let mut memory = self.clone();
        memory.defragment_fractional_files();
        memory.checksum()
    }
    fn calculate_part2(&self) -> Self::Output {
        let mut memory = self.clone();
        memory.defragment_whole_files();
        memory.checksum()
    }
}

fn main() { execute_day::<Day>() }

#[cfg(test)]
mod tests {
    use advent_lib::day_test;

    day_test!( 9, example1 => 1928, 2858 );
    day_test!( 9 => 6258319840548, 6286182965311);
}
