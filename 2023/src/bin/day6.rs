#![feature(test)]

use advent_lib::day::*;

struct Day {
    races: Vec<Race>,
}

#[derive(Debug, Default, Copy, Clone)]
struct Race {
    time: u64,
    distance: u64,
}

impl Race {
    fn solve(&self) -> u64 {
        let time = self.time as f64;
        let dist = self.distance as f64;
        let s = (time * time - 4f64 * dist).sqrt();
        let min = (time - s) / 2f64;
        let max = (time + s) / 2f64;
        let min = if min.ceil() == min { min as u64 + 1 } else { min.ceil() as u64 };
        let max = if max.floor() == max { max as u64 - 1 } else { max.floor() as u64 };
        max - min + 1
    }

    fn combine(self, other: &Race) -> Race {
        Race {
            time: (self.time.to_string() + &other.time.to_string()).parse().unwrap(),
            distance: (self.distance.to_string() + &other.distance.to_string()).parse().unwrap(),
        }
    }
}

impl ExecutableDay for Day {
    type Output = u64;

    fn from_lines<LINES: Iterator<Item = String>>(mut lines: LINES) -> Self {
        let times = lines
            .next()
            .unwrap()
            .strip_prefix("Time:     ")
            .unwrap()
            .split(' ')
            .flat_map(|nr| nr.parse().ok())
            .collect::<Vec<_>>();
        let distances = lines
            .next()
            .unwrap()
            .strip_prefix("Distance: ")
            .unwrap()
            .split(' ')
            .flat_map(|nr| nr.parse().ok())
            .collect::<Vec<_>>();

        Day {
            races: times
                .iter()
                .zip(distances.iter())
                .map(|(&time, &distance)| Race { time, distance })
                .collect(),
        }
    }

    fn calculate_part1(&self) -> Self::Output { self.races.iter().map(Race::solve).product() }

    fn calculate_part2(&self) -> Self::Output {
        self.races.iter().fold(Default::default(), Race::combine).solve()
    }
}

fn main() { execute_day::<Day>() }

#[cfg(test)]
mod tests {
    use advent_lib::day_test;

    day_test!( 6, example => 288, 71503);
    day_test!( 6 => 1731600, 40087680 );
}
