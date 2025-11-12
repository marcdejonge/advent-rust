#!/usr/bin/env bash

if [ "$#" -ne 2 ]; then
    echo "Usage: $0 <year> <day_number>"
    exit 1
fi

YEAR=$1
DAY_NUM=$2

if ! [[ "$YEAR" =~ ^[0-9]{4}$ ]]; then
    echo "Year must be a four-digit number."
    exit 1
fi

if ! [[ "$DAY_NUM" =~ ^[0-9]+$ ]] || [ "$DAY_NUM" -lt 1 ] || [ "$DAY_NUM" -gt 25 ]; then
    echo "Day number must be an integer between 1 and 25."
    exit 1
fi

if [ ! -d "$YEAR" ]; then
    echo "Year directory '$YEAR' does not exist."
    exit 1
fi

if [ ! -d "$YEAR/src/bin" ]; then
    echo "Source bin directory '$YEAR/src/bin' does not exist."
    exit 1
fi

if [ ! -d "$YEAR/input" ]; then
    echo "Input directory '$YEAR/input' does not exist."
    exit 1
fi

if [ -f "${YEAR}/src/bin/day${DAY_NUM}.rs" ]; then
    echo "Day file '${YEAR}/src/bin/day${DAY_NUM}.rs' already exists."
    exit 1
fi

touch "${YEAR}/input/day${DAY_NUM}.txt"
touch "${YEAR}/input/day${DAY_NUM}_example.txt"

cat <<EOF > "${YEAR}/src/bin/day${DAY_NUM}.rs"
#![feature(test)]

use advent_lib::*;
use nom_parse_macros::parse_from;

#[parse_from]
struct Input {
    dummy: Vec<u32>
}

fn calculate_part1(input: &Input) -> u64 {
    todo!()
}

fn calculate_part2(input: &Input) -> u64 {
    todo!()
}

day_main!(Input);

day_test!( $DAY_NUM, example => 0 );
day_test!( $DAY_NUM => 0 );

EOF