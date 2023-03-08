use bit_set::BitSet;

crate::day!(8, Grid<u8>, usize {
    parse_input(input) {
        let heights: Vec<Vec<u8>> = input.lines().map(|line|
            line.bytes()
                .filter_map(|byte|
                    if (b'0'..=b'9').contains(&byte) {
                        Some(byte - b'0' + 1)
                    } else {
                        None
                    }
                ).collect()
        ).collect();
        let height = heights.len();
        let width = heights[0].len();
        if heights.iter().any(|line| line.len() != width) {
            panic!("Not all input lines have the same length");
        }
        Grid { items: heights.iter().flatten().cloned().collect(), height, width }
    }

    calculate_part1(tree_heights) {
        let mut set = BitSet::new();
        tree_heights.all_lines_both().for_each(|line| {
            let mut max = 0u8;
            for (ix, height) in line {
                if *height > max {
                    max = *height;
                    set.insert(ix);
                }
            }
        });
        set.len()
    }

    calculate_part2(tree_heights) {
        let mut scores = vec![1usize; tree_heights.items.len()];
        tree_heights.all_lines_both().for_each(|line| {
            let mut last_seen = [0usize; 11];
            let mut step = 0usize;
            for (ix, &height) in line {
                scores[ix] *= step - last_seen[height as usize];
                for blocked_height in 0..=height {
                    last_seen[blocked_height as usize] = step
                }
                step += 1;
            }
        });
        scores.iter().max().cloned().unwrap()
    }

    test example_input(include_str!("example_input/day8.txt") => 21, 8)
});

#[derive(Debug)]
struct Grid<T> {
    items: Vec<T>,
    height: usize,
    width: usize,
}

#[derive(Debug, Clone)]
struct GridLine<'a, T> {
    grid: &'a Grid<T>,
    ix: usize,
    d_ix: usize,
    limit_ix: usize,
    reversed: Option<usize>,
}

impl<'a, T> GridLine<'a, T> {
    fn reverse(self) -> GridLine<'a, T> {
        GridLine {
            reversed: Some(self.limit_ix + self.ix),
            ..self
        }
    }
}

impl<'a, T> Iterator for GridLine<'a, T> {
    type Item = (usize, &'a T);

    fn next(&mut self) -> Option<Self::Item> {
        if self.ix > self.limit_ix {
            None
        } else {
            let next_ix = if let Some(top) = self.reversed {
                top - self.ix
            } else {
                self.ix
            };
            let ret = Some((next_ix, self.grid.items.get(next_ix).unwrap()));
            self.ix += self.d_ix;
            ret
        }
    }
}

impl<T> Grid<T> {
    fn create_line(&self, start_ix: usize, step_size: usize, steps: usize) -> GridLine<T> {
        GridLine {
            grid: self,
            ix: start_ix,
            d_ix: step_size,
            limit_ix: start_ix + (steps - 1) * step_size,
            reversed: None,
        }
    }

    fn horizontal_lines(&self) -> impl Iterator<Item = GridLine<T>> {
        (0..self.height).map(|y| self.create_line(y * self.width, 1, self.height))
    }

    fn vertical_lines(&self) -> impl Iterator<Item = GridLine<T>> {
        (0..self.width).map(|x| self.create_line(x, self.width, self.height))
    }

    fn all_lines(&self) -> impl Iterator<Item = GridLine<T>> {
        self.horizontal_lines().chain(self.vertical_lines())
    }

    fn all_lines_both(&self) -> impl Iterator<Item = GridLine<T>> {
        self.all_lines()
            .chain(self.all_lines().map(|line| line.reverse()))
    }
}
