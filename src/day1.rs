use std::collections::BinaryHeap;

use crate::iter_utils::Chunkable;

crate::day!(1, Vec<i32>, i32 {
    parse_input(input) {
        input.lines()
            .chunk_by("")
            .map(|v| v.iter().map(|&line| line.parse::<i32>().unwrap()).sum())
            .collect::<BinaryHeap<_>>()
            .into_sorted_vec()
    }

    calculate_part1(input) {
        input.iter().rev().take(1).sum()
    }

    calculate_part2(input) {
        input.iter().rev().take(3).sum()
    }

    example_input("1000\n2000\n3000\n\n4000\n\n5000\n6000\n\n7000\n8000\n9000\n\n10000" => 24000, 45000)
});
