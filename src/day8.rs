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
        let mut ctx = Part1Context { is_reachable: BitSet::new(), max: 0u8 };
        tree_heights.for_all_lines(&mut ctx, |ctx| {
            ctx.max = 0u8;
        }, |ctx, &height, ix| {
                if height > ctx.max {
                    ctx.max = height;
                    ctx.is_reachable.insert(ix);
                }
            })
        .iter().fold(BitSet::new(), |mut acc, ctx| {
            acc.union_with(&ctx.is_reachable);
            acc
        }).len()
    }

    calculate_part2(tree_heights) {
        // The context contains the scores, the last_seen and the current step
        let ctx = Part2Context { scores: vec![1usize; tree_heights.len()], last_seen: [0usize; 11], step: 0usize};
        tree_heights.for_all_lines(&ctx, |ctx| {
            ctx.last_seen.iter_mut().for_each(|x| *x = 0);
            ctx.step = 0;
        }, |ctx, &height, ix| {
                ctx.scores[ix] = ctx.step - ctx.last_seen[height as usize];
                for blocked_height in 0..=height {
                    ctx.last_seen[blocked_height as usize] = ctx.step
                }
                ctx.step += 1;
        })
        .iter().fold(ctx.scores, |mut acc, ctx| {
            for ix in 0..tree_heights.len() {
                acc[ix] *= ctx.scores[ix];
            }
            acc
        }).iter().max().cloned().unwrap()
    }

    test example_input(include_str!("example_input/day8.txt") => 21, 8)
});

#[derive(Clone)]
struct Part1Context {
    is_reachable: BitSet,
    max: u8,
}

#[derive(Clone)]
struct Part2Context {
    scores: Vec<usize>,
    last_seen: [usize; 11],
    step: usize,
}
