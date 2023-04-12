#![feature(test)]
use advent_lib::day::*;
use fxhash::FxBuildHasher;
use std::collections::HashSet;

struct Day {
    grid: Vec<Vec<u32>>,
}

impl FromIterator<String> for Day {
    fn from_iter<T: IntoIterator<Item = String>>(iter: T) -> Self {
        Day {
            grid: iter
                .into_iter()
                .map(|line| {
                    line.chars()
                        .map(|c| match c {
                            'a'..='z' => c as u32 - 'a' as u32 + 1,
                            'A'..='Z' => c as u32 - 'A' as u32 + 27,
                            _ => 0,
                        })
                        .collect()
                })
                .collect(),
        }
    }
}

impl ExecutableDay for Day {
    type Output = u32;

    fn calculate_part1(&self) -> Self::Output {
        self.grid
            .iter()
            .map(|line| {
                let (left, right) = line.split_at(line.len() / 2);
                let set: HashSet<_, FxBuildHasher> = left.iter().copied().collect();
                right.iter().filter(|&c| set.contains(c)).last().unwrap()
            })
            .sum()
    }

    fn calculate_part2(&self) -> Self::Output {
        self.grid
            .chunks(3)
            .map(|lines| {
                let set = lines[0]
                    .iter()
                    .copied()
                    .collect::<HashSet<_>>()
                    .intersection(&lines[1].iter().copied().collect())
                    .copied()
                    .collect::<HashSet<_, FxBuildHasher>>();
                lines[2].iter().filter(|&c| set.contains(c)).last().unwrap()
            })
            .sum()
    }
}

fn main() { execute_day::<Day>() }

#[cfg(test)]
mod tests {
    use advent_lib::day_test;

    day_test!( 3, example => 157, 70 );
    day_test!( 3 => 8123, 2620 );
}
