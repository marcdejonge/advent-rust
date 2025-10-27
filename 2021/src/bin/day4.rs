#![feature(test)]

use advent_lib::{
    day_main, day_test,
    parsing::{double_line_ending, separated_array, separated_array_with},
};
use nom_parse_macros::parse_from;

const MARKED: u32 = u32::MAX;

#[parse_from(separated_pair(separated_list1(",", u32), double_line_ending, separated_list1(double_line_ending, {})))]
struct Input {
    numbers: Vec<u32>,
    boards: Vec<Board>,
}

#[parse_from(separated_array_with(line_ending, preceded(space0, separated_array(space1))))]
#[derive(Clone)]
struct Board([[u32; 5]; 5]);

impl Board {
    fn mark_number(&mut self, nr: u32) -> Option<(usize, usize)> {
        for row in 0..5 {
            for col in 0..5 {
                if self.0[row][col] == nr {
                    self.0[row][col] = MARKED;
                    return Some((row, col));
                }
            }
        }
        return None;
    }

    fn has_bingo(&self, row: usize, col: usize) -> bool {
        self.0[row].iter().all(|&cell| cell == MARKED)
            || (0..5).all(|row| self.0[row][col] == MARKED)
    }

    fn unmarked_sum(&self) -> u32 {
        self.0
            .iter()
            .flat_map(|row| row.iter())
            .filter(|&&cell| cell != MARKED)
            .sum()
    }
}

fn calculate_part1(input: &Input) -> u32 {
    let mut boards = input.boards.clone();
    for &number in &input.numbers {
        for board in &mut boards {
            if let Some((row, col)) = board.mark_number(number) {
                if board.has_bingo(row, col) {
                    return board.unmarked_sum() * number;
                }
            }
        }
    }
    unreachable!()
}

fn calculate_part2(input: &Input) -> u32 {
    let mut boards = input.boards.clone();
    for &number in &input.numbers {
        if boards.len() == 1 {
            let board = &mut boards[0];
            if let Some((row, col)) = board.mark_number(number) {
                if board.has_bingo(row, col) {
                    return board.unmarked_sum() * number;
                }
            }
        } else {
            boards.retain_mut(|board| {
                if let Some((row, col)) = board.mark_number(number) {
                    !board.has_bingo(row, col)
                } else {
                    true
                }
            });
        }
    }
    unreachable!()
}

day_main!();

day_test!( 4, example => 4512, 1924 );
day_test!( 4 => 46920, 12635 );
