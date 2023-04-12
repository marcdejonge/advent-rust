#![feature(test)]
use advent_lib::day::*;
use advent_lib::grid::Grid;
use bit_set::BitSet;

struct Day {
    tree_heights: Grid<u8>,
}

impl FromIterator<String> for Day {
    fn from_iter<T: IntoIterator<Item = String>>(iter: T) -> Self {
        Day {
            tree_heights: Grid::new(
                iter.into_iter()
                    .map(|line| {
                        line.chars()
                            .filter_map(|c| {
                                if ('0'..='9').contains(&c) {
                                    Some((c as u8) - b'0' + 1)
                                } else {
                                    None
                                }
                            })
                            .collect()
                    })
                    .collect(),
            ),
        }
    }
}

impl ExecutableDay for Day {
    type Output = usize;

    fn calculate_part1(&self) -> Self::Output {
        let mut reachable = BitSet::new();

        self.tree_heights.for_all_lines(
            &mut reachable,
            &0u8,
            |reachable, max, &height, ix| {
                // for each cell
                if height > *max {
                    *max = height;
                    reachable.insert(ix);
                }
            },
            |reachable, thread_reachable| {
                // combine thread results
                reachable.union_with(&thread_reachable);
            },
        );

        reachable.len()
    }

    fn calculate_part2(&self) -> Self::Output {
        let mut scores = vec![1usize; self.tree_heights.len()];
        let last_seen = [0usize; 11];

        self.tree_heights.for_all_lines(
            &mut scores,
            &last_seen,
            |scores, last_seen, &height, ix| {
                // for each cell
                scores[ix] = last_seen[0] - last_seen[height as usize];
                for blocked_height in 0..=height {
                    last_seen[blocked_height as usize] = last_seen[0]
                }
                last_seen[0] += 1;
            },
            |scores, thread_scores| {
                // combine thread results
                for ix in 0..scores.len() {
                    scores[ix] *= thread_scores[ix];
                }
            },
        );

        scores.iter().max().cloned().unwrap()
    }
}

fn main() { execute_day::<Day>() }

#[cfg(test)]
mod tests {
    use advent_lib::day_test;

    day_test!( 8, example => 21, 8 );
    day_test!( 8 => 1700, 470596 );
}
