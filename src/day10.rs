crate::day!(10, Vec<i32>, String {
    parse_input(input) {
        let mut result = Vec::new();
        let mut x = 1i32;
        input.lines().for_each(|line| {
            result.push(x);
            if line.starts_with("addx ") {
                result.push(x);
                x += line[5..].parse::<i32>().expect("Expected number");
            }
        });
        result
    }

    calculate_part1(input) {
        input.iter().enumerate()
            .filter(|(time, _)| time % 40 == 19)
            .map(|(time, x)| (time as i32 + 1) * x)
            .sum::<i32>().to_string()
    }

    calculate_part2(input) {
        let mut screen = String::with_capacity(256);
        input.iter().enumerate().for_each(|(time, x)| {
            if (time % 40) == 0 { screen.push('\n'); }
            screen.push(if (x - (time as i32 % 40)).abs() <= 1 { '#' } else { '.' });
        });
        screen
    }

    test example_input(include_str!("example_input/day10.txt") => "13140", "
##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######.....")
});
