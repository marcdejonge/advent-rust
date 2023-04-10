use advent_lib::day::*;
use regex::Regex;
use std::ops::RangeInclusive;

struct Day {
    range_pairs: Vec<(RangeInclusive<u32>, RangeInclusive<u32>)>,
}

impl FromIterator<String> for Day {
    fn from_iter<T: IntoIterator<Item = String>>(iter: T) -> Self {
        let re = Regex::new("(\\d+)-(\\d+),(\\d+)-(\\d+)").unwrap();
        Day {
            range_pairs: iter
                .into_iter()
                .map(|line| {
                    let cap = re.captures(&line).unwrap();
                    (
                        RangeInclusive::new(cap[1].parse().unwrap(), cap[2].parse().unwrap()),
                        RangeInclusive::new(cap[3].parse().unwrap(), cap[4].parse().unwrap()),
                    )
                })
                .collect(),
        }
    }
}

impl ExecutableDay for Day {
    type Output = usize;

    fn calculate_part1(&self) -> Self::Output {
        self.range_pairs
            .iter()
            .filter(|(first, second)| is_contained(first, second) || is_contained(second, first))
            .count()
    }

    fn calculate_part2(&self) -> Self::Output {
        self.range_pairs
            .iter()
            .filter(|(first, second)| {
                first.contains(&second.start())
                    || first.contains(&(second.end()))
                    || second.contains(&first.start())
                    || second.contains(&(first.end()))
            })
            .count()
    }
}

fn is_contained<T: PartialOrd<T>>(outside: &RangeInclusive<T>, inside: &RangeInclusive<T>) -> bool {
    outside.start() <= inside.start() && outside.end() >= inside.end()
}

fn main() { execute_day::<Day>() }

#[cfg(test)]
mod tests {
    use advent_lib::day_test;

    day_test!( 4, example => 2, 4 );
    day_test!( 4 => 580, 895 );
}
