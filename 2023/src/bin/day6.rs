/*
--- Day 6: Wait For It ---

The ferry quickly brings you across Island Island. After asking around, you discover that there is
indeed normally a large pile of sand somewhere near here, but you don't see anything besides lots of
water and the small island where the ferry has docked.

As you try to figure out what to do next, you notice a poster on a wall near the ferry dock. "Boat
races! Open to the public! Grand prize is an all-expenses-paid trip to Desert Island!" That must be
where the sand comes from! Best of all, the boat races are starting in just a few minutes.

You manage to sign up as a competitor in the boat races just in time. The organizer explains that
it's not really a traditional race - instead, you will get a fixed amount of time during which your
boat has to travel as far as it can, and you win if your boat goes the farthest.

As part of signing up, you get a sheet of paper (your puzzle input) that lists the time allowed for
each race and also the best distance ever recorded in that race. To guarantee you win the grand
prize, you need to make sure you go farther in each race than the current record holder.

The organizer brings you over to the area where the boat races are held. The boats are much smaller
than you expected - they're actually toy boats, each with a big button on top. Holding down the
button charges the boat, and releasing the button allows the boat to move. Boats move faster if
their button was held longer, but time spent holding the button counts against the total race time.
You can only hold the button at the start of the race, and boats don't move until the button is
released.

To see how much margin of error you have, determine the number of ways you can beat the record in
each race.

Determine the number of ways you could beat the record in each race. What do you get if you
multiply these numbers together?

--- Part Two ---

As the race is about to start, you realize the piece of paper with race times and record distances
you got earlier actually just has very bad kerning. There's really only one race - ignore the spaces
between the numbers on each line.

How many ways can you beat the record in this one much longer race?
*/

#![feature(test)]

use advent_lib::day::{execute_day, ExecutableDay};

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
