crate::day!(2, Vec<String>, i32 {
    parse_input(file_input) {
        file_input.lines().map(|line| line.to_owned()).collect()
    }

    calculate_part1(input) {
        input.iter().map(|line| match line.as_str() {
            "A X" => 4,
            "A Y" => 8,
            "A Z" => 3,
            "B X" => 1,
            "B Y" => 5,
            "B Z" => 9,
            "C X" => 7,
            "C Y" => 2,
            "C Z" => 6,
            _ => panic!("Unexpected game {}", line)
        }).sum()
    }

    calculate_part2(input) {
        input.iter().map(|line| match line.as_str() {
            "A X" => 3,
            "A Y" => 4,
            "A Z" => 8,
            "B X" => 1,
            "B Y" => 5,
            "B Z" => 9,
            "C X" => 2,
            "C Y" => 6,
            "C Z" => 7,
            _ => panic!("Unexpected game {}", line)
        }).sum()
    }

    test example_input("A Y\nB X\nC Z" => 15, 12)
});
