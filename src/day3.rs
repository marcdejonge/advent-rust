use std::collections::HashSet;

crate::day!(3, Vec<Vec<u32>>, u32 {
    parse_input(file_input) {
        file_input.lines().map(|line| line.chars().map(|c| match c {
            'a'..='z' => c as u32 - 'a' as u32 + 1,
            'A'..='Z' => c as u32 - 'A' as u32 + 27,
            _ => 0
        }).collect()).collect()
    }

    calculate_part1(input) {
        input.iter().map(|line| {
            let (left, right) = line.split_at(line.len() / 2);
            let set: HashSet<_> = left.iter().copied().collect();
            right.iter().filter(|&c| set.contains(c)).last().unwrap()
        }).sum()
    }

    calculate_part2(input) {
        input.chunks(3).map(|lines| {
            let set = lines[0].iter().copied().collect::<HashSet<_>>()
                .intersection(&lines[1].iter().copied().collect())
                .copied().collect::<HashSet<_>>();
            lines[2].iter().filter(|&c| set.contains(c)).last().unwrap()
        }).sum()
    }

    test example_input(include_str!("example_input/day3.txt") => 157, 70)
});
