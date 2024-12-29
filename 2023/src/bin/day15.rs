#![feature(test)]

use std::hash::Hasher;

use advent_lib::day_main;
use advent_macros::parsable;

#[parsable(separated_list1(tag(b","), Operation::parser()))]
struct Input {
    words: Vec<Operation>,
}

#[parsable]
enum Operation {
    #[format=terminated(map_to_vec(alpha1), tag(b"-"))]
    Remove(Vec<u8>),
    #[format=separated_pair(map_to_vec(alpha1), tag(b"="), u64)]
    Set(Vec<u8>, u64),
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

    fn hash_op(operation: &Operation) -> u64 {
        match operation {
            Operation::Remove(key) => {
                let mut hasher = ShortHasher { curr: 0 };
                hasher.write(key);
                hasher.write(b"-");
                hasher.finish()
            }
            Operation::Set(key, value) => {
                let mut hasher = ShortHasher { curr: 0 };
                hasher.write(key);
                hasher.write(b"=");
                hasher.write(value.to_string().as_bytes());
                hasher.finish()
            }
        }
    }
}

fn calculate_part1(input: &Input) -> u64 { input.words.iter().map(ShortHasher::hash_op).sum() }

fn calculate_part2(input: &Input) -> u64 {
    const VEC: Vec<(&[u8], u64)> = Vec::new();
    let mut boxes = [VEC; 256];

    input.words.iter().for_each(|w| match w {
        Operation::Remove(search_key) => {
            let hash = ShortHasher::hash64(search_key) as usize;
            boxes[hash].retain(|(key, _)| key != search_key);
        }
        Operation::Set(key, value) => {
            let hash = ShortHasher::hash64(key) as usize;

            let mut found = false;
            for (stored_key, stored_value) in &mut boxes[hash] {
                if *stored_key == key {
                    *stored_value = *value;
                    found = true;
                    break;
                }
            }
            if !found {
                boxes[hash].push((key, *value));
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

day_main!();

#[cfg(test)]
mod tests {
    use advent_lib::day_test;

    day_test!( 15, example => 1320, 145 );
    day_test!( 15 => 505379, 263211 );
}
