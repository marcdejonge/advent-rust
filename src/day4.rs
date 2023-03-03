use std::ops::RangeInclusive;

use regex::Regex;

crate::day!(4, Vec<(RangeInclusive<u32>, RangeInclusive<u32>)>, usize {
    parse_input(file_input) {
        let re = Regex::new("(\\d+)-(\\d+),(\\d+)-(\\d+)").unwrap();
        file_input.lines().map(|line| {
            let cap = re.captures(line).unwrap();
            (
                RangeInclusive::new(cap[1].parse().unwrap(), cap[2].parse().unwrap()),
                RangeInclusive::new(cap[3].parse().unwrap(), cap[4].parse().unwrap())
            )
        }).collect()
    }

    calculate_part1(input) {
        input.iter().filter(|(first, second)| {
            is_contained(first, second) || is_contained(second, first)
        }).count()
    }

    calculate_part2(input) {
        input.iter().filter(|(first, second)| {
            first.contains(&second.start())
                || first.contains(&(second.end()))
                || second.contains(&first.start())
                || second.contains(&(first.end()))
        }).count()
    }

    test example_input("2-4,6-8\n2-3,4-5\n5-7,7-9\n2-8,3-7\n6-6,4-6\n2-6,4-8" => 2, 4)
});

fn is_contained<T: PartialOrd<T>>(outside: &RangeInclusive<T>, inside: &RangeInclusive<T>) -> bool {
    outside.start() <= inside.start() && outside.end() >= inside.end()
}
