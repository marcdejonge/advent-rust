use advent_lib::day::{execute_day, ExecutableDay};

struct Day {
    additions: Vec<i32>,
}

impl FromIterator<String> for Day {
    fn from_iter<T: IntoIterator<Item = String>>(iter: T) -> Self {
        let mut additions = Vec::new();
        let mut x = 1i32;
        iter.into_iter().for_each(|line| {
            additions.push(x);
            if line.starts_with("addx ") {
                additions.push(x);
                x += line[5..].parse::<i32>().expect("Expected number");
            }
        });
        Day { additions }
    }
}

impl ExecutableDay for Day {
    type Output = String;

    fn calculate_part1(&self) -> Self::Output {
        self.additions
            .iter()
            .enumerate()
            .filter(|(time, _)| time % 40 == 19)
            .map(|(time, x)| (time as i32 + 1) * x)
            .sum::<i32>()
            .to_string()
    }

    fn calculate_part2(&self) -> Self::Output {
        let mut screen = String::with_capacity(256);
        self.additions.iter().enumerate().for_each(|(time, x)| {
            if (time % 40) == 0 {
                screen.push('\n');
            }
            screen.push(if (x - (time as i32 % 40)).abs() <= 1 { '#' } else { '.' });
        });
        screen
    }
}

fn main() { execute_day::<Day>() }

#[cfg(test)]
mod tests {
    use advent_lib::day_test;

    day_test!( 10, example => "13140".to_owned(), "
##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######.....".to_owned() );
    day_test!( 10 => "15260".to_owned(), "
###...##..#..#.####..##..#....#..#..##..
#..#.#..#.#..#.#....#..#.#....#..#.#..#.
#..#.#....####.###..#....#....#..#.#....
###..#.##.#..#.#....#.##.#....#..#.#.##.
#....#..#.#..#.#....#..#.#....#..#.#..#.
#.....###.#..#.#.....###.####..##...###.".to_owned() );
}
