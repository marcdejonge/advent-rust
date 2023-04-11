use advent_lib::day::*;
use prse::Parse;
use prse_derive::parse;

struct Day {
    range_pairs: Vec<(Range, Range)>,
}

#[derive(Parse)]
#[prse = "{from}-{to}"]
struct Range {
    from: u32,
    to: u32,
}

impl Range {
    fn contains(&self, value: u32) -> bool { value >= self.from && value <= self.to }
    fn wraps(&self, other: &Range) -> bool { self.from <= other.from && self.to >= other.to }
}

impl FromIterator<String> for Day {
    fn from_iter<T: IntoIterator<Item = String>>(iter: T) -> Self {
        Day {
            range_pairs: iter.into_iter().map(|line: String| parse!(line, "{},{}")).collect(),
        }
    }
}

impl ExecutableDay for Day {
    type Output = usize;

    fn calculate_part1(&self) -> Self::Output {
        self.range_pairs
            .iter()
            .filter(|(first, second)| first.wraps(second) || second.wraps(first))
            .count()
    }

    fn calculate_part2(&self) -> Self::Output {
        self.range_pairs
            .iter()
            .filter(|(first, second)| {
                first.contains(second.from)
                    || first.contains(second.to)
                    || second.contains(first.from)
                    || second.contains(first.to)
            })
            .count()
    }
}

fn main() { execute_day::<Day>() }

#[cfg(test)]
mod tests {
    use advent_lib::day_test;

    day_test!( 4, example => 2, 4 );
    day_test!( 4 => 580, 895 );
}
