use bit_set::BitSet;

use crate::grid::Grid;

crate::day!(8, Grid<u8>, usize {
    parse_input(input) {
        Grid::new(input.lines().map(|line|
            line.bytes()
                .filter_map(|byte|
                    if (b'0'..=b'9').contains(&byte) {
                        Some(byte - b'0' + 1)
                    } else {
                        None
                    }
                ).collect()
        ).collect())
    }

    calculate_part1(tree_heights) {
        let mut reachable = BitSet::new();

        tree_heights.for_all_lines(&mut reachable, &0u8,
            |reachable, max, &height, ix| { // for each cell
                if height > *max {
                    *max = height;
                    reachable.insert(ix);
                }
            },
            |reachable, thread_reachable| { // combine thread results
                reachable.union_with(&thread_reachable);
            }
        );

        reachable.len()
    }

    calculate_part2(tree_heights) {
        let mut scores = vec![1usize; tree_heights.len()];
        let last_seen = [0usize; 11];

        tree_heights.for_all_lines(&mut scores, &last_seen,
            |scores, last_seen, &height, ix| { // for each cell
                scores[ix] = last_seen[0] - last_seen[height as usize];
                for blocked_height in 0..=height {
                    last_seen[blocked_height as usize] = last_seen[0]
                }
                last_seen[0] += 1;
            },
            |scores, thread_scores| { // combine thread results
                for ix in 0..scores.len() {
                    scores[ix] *= thread_scores[ix];
                }
            }
        );

        scores.iter().max().cloned().unwrap()
    }

    test example_input(include_str!("example_input/day8.txt") => 21, 8)
});
