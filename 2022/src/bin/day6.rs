use advent_lib::day::*;

struct Day {
    bytes: Vec<u8>,
}

impl FromIterator<String> for Day {
    fn from_iter<T: IntoIterator<Item = String>>(iter: T) -> Self {
        Day {
            bytes: iter
                .into_iter()
                .next()
                .expect("Expected a single line")
                .chars()
                .filter_map(
                    |c| {
                        if ('a'..='z').contains(&c) {
                            Some((c as u8) - 97)
                        } else {
                            None
                        }
                    },
                )
                .collect(),
        }
    }
}

impl ExecutableDay for Day {
    type Output = u32;

    fn calculate_part1(&self) -> Self::Output {
        self.find(4).expect("Could not find result for part 1")
    }

    fn calculate_part2(&self) -> Self::Output {
        self.find(14).expect("Could not find result for part 2")
    }
}

impl Day {
    fn find(&self, size: u32) -> Option<u32> {
        let mut start_iter = self.bytes.iter();
        let mut end_iter = self.bytes.iter();
        let mut mask: u32 = 0;
        let mut count: u32 = 0;

        for _ in 0..size {
            mask ^= 1u32 << end_iter.next()?;
            count += 1;
        }

        while mask.count_ones() != size {
            mask ^= 1u32 << end_iter.next()?;
            mask ^= 1u32 << start_iter.next()?;
            count += 1;
        }

        Some(count)
    }
}

fn main() { execute_day::<Day>() }

#[cfg(test)]
mod tests {
    use advent_lib::day_test;

    day_test!( 6, example1 => 7, 19 );
    day_test!( 6, example2 => 5, 23 );
    day_test!( 6, example3 => 6, 23 );
    day_test!( 6, example4 => 10, 29 );
    day_test!( 6, example5 => 11, 26 );
    day_test!( 6 => 1235, 3051 );
}
