#![feature(test)]

use std::hash::Hasher;

use advent_lib::day::*;

struct Day {
    words: Vec<String>,
}

struct ShortHasher {
    curr: u64,
}

impl Hasher for ShortHasher {
    fn finish(&self) -> u64 { self.curr }

    fn write(&mut self, bytes: &[u8]) {
        for b in bytes {
            self.curr = ((self.curr + (*b as u64)) * 17) & 0xff;
        }
    }
}

impl ShortHasher {
    fn hash64(bytes: &[u8]) -> u64 {
        let mut hasher = ShortHasher { curr: 0 };
        hasher.write(bytes);
        hasher.finish()
    }
}

impl ExecutableDay for Day {
    type Output = u64;

    fn from_lines<LINES: Iterator<Item = String>>(mut lines: LINES) -> Self {
        Day { words: lines.next().unwrap().split(',').map(str::to_owned).collect() }
    }

    fn calculate_part1(&self) -> Self::Output {
        self.words.iter().map(|w| ShortHasher::hash64(w.as_bytes())).sum()
    }

    fn calculate_part2(&self) -> Self::Output {
        const VEC: Vec<(&str, u64)> = Vec::new();
        let mut boxes = [VEC; 256];

        self.words.iter().for_each(|w| {
            if w.ends_with('-') {
                let search_key = &w[0..w.len() - 1];
                let hash = ShortHasher::hash64(search_key.as_bytes()) as usize;
                boxes[hash].retain(|(key, _)| *key != search_key);
            } else {
                let (key, value) = w.split_at(w.find('=').unwrap());
                let value = value[1..].parse().unwrap();
                let hash = ShortHasher::hash64(key.as_bytes()) as usize;

                let mut found = false;
                for (stored_key, stored_value) in &mut boxes[hash] {
                    if *stored_key == key {
                        *stored_value = value;
                        found = true;
                        break;
                    }
                }
                if !found {
                    boxes[hash].push((key, value));
                }
            }
        });

        boxes
            .iter()
            .enumerate()
            .flat_map(|(box_ix, list)| {
                list.iter()
                    .enumerate()
                    .map(move |(ix, (_, value))| (box_ix as u64 + 1) * (ix as u64 + 1) * (*value))
            })
            .sum()
    }
}

fn main() { execute_day::<Day>() }

#[cfg(test)]
mod tests {
    use advent_lib::day_test;

    day_test!( 15, example => 1320, 145 );
    day_test!( 15 => 505379, 263211 );
}
