crate::day!(6, Vec<u8>, u32 {
   parse_input(input) {
        input.bytes().filter_map(|b| {
            if (97..=122).contains(&b) { Some(b - 97) } else { None }
        }).collect()
    }

    calculate_part1(input) {
        find(input, 4).expect("Could not find result for part 1")
    }

    calculate_part2(input) {
        find(input, 14).expect("Could not find result for part 2")
    }

    test example1("mjqjpqmgbljsphdztnvjfqwrcgsmlb" => 7, 19)
    test example2("bvwbjplbgvbhsrlpgdmjqwftvncz" => 5, 23)
    test example3("nppdvjthqldpwncqszvftbrmjlhg" => 6, 23)
    test example4("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg" => 10, 29)
    test example5("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw" => 11, 26)
});

fn find(input: &Vec<u8>, size: u32) -> Option<u32> {
    let mut start_iter = input.iter();
    let mut end_iter = input.iter();
    let mut mask: u32 = 0;
    let mut count: u32 = 0;

    for _ in 0..size {
        mask ^= 1u32 << end_iter.next()?;
        count += 1;
    }

    while mask.count_ones() != size {
        mask ^= 1u32 << end_iter.next()?;
        mask ^= 1u32 << start_iter.next()?;
        count += 1;
    }

    Some(count)
}