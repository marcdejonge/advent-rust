use std::fmt::{Debug, Formatter, Write};
use std::ops::RangeInclusive;

use crossbeam::scope;

#[derive(Clone)]
pub struct Grid<T> {
    items: Vec<T>,
    x_indices: RangeInclusive<i32>,
    y_indices: RangeInclusive<i32>,
    width: usize,
    height: usize,
}

impl<T: Clone + Default> Grid<T> {
    pub fn new_empty(x_indices: RangeInclusive<i32>, y_indices: RangeInclusive<i32>) -> Grid<T> {
        let width = (x_indices.end() - x_indices.start() + 1) as usize;
        let height = (y_indices.end() - y_indices.start() + 1) as usize;
        Grid {
            items: vec![Default::default(); width * height],
            x_indices,
            y_indices,
            width,
            height,
        }
    }
}

impl<T> Grid<T> {
    pub fn new(items: Vec<Vec<T>>) -> Grid<T>
    where
        T: Clone,
    {
        let height = items.len();
        let width = items[0].len();
        if items.iter().any(|line| line.len() != width) {
            panic!("Not all input lines have the same length");
        }
        Grid {
            items: items.iter().flatten().cloned().collect(),
            height,
            width,
            x_indices: (0..=(width - 1) as i32),
            y_indices: (0..=(height - 1) as i32),
        }
    }

    #[inline]
    pub fn for_all_lines<C, LC>(
        &self,
        result: &mut C,
        line_context: &LC,
        each_cell: fn(&mut C, &mut LC, &T, usize),
        combine: fn(&mut C, C),
    ) where
        C: Clone + Sync + Send,
        LC: Clone + Sync,
        T: Sync + Send,
    {
        let thread_results = scope(|s| {
            [
                s.spawn(|_| {
                    let mut ctx = result.clone();
                    for y in 0..self.height {
                        let mut lc = line_context.clone();
                        for index in (y * self.width)..((y + 1) * self.width) {
                            each_cell(&mut ctx, &mut lc, self.items.get(index).unwrap(), index)
                        }
                    }
                    ctx
                }),
                s.spawn(|_| {
                    let mut ctx = result.clone();
                    for y in 0..self.height {
                        let mut lc = line_context.clone();
                        for index in ((y * self.width)..((y + 1) * self.width)).rev() {
                            each_cell(&mut ctx, &mut lc, self.items.get(index).unwrap(), index)
                        }
                    }
                    ctx
                }),
                s.spawn(|_| {
                    let mut ctx = result.clone();
                    for x in 0..self.width {
                        let mut lc = line_context.clone();
                        for index in (x..(self.height * self.width)).step_by(self.width) {
                            each_cell(&mut ctx, &mut lc, self.items.get(index).unwrap(), index)
                        }
                    }
                    ctx
                }),
                s.spawn(|_| {
                    let mut ctx = result.clone();
                    for x in 0..self.width {
                        let mut lc = line_context.clone();
                        for index in (x..(self.height * self.width)).step_by(self.width).rev() {
                            each_cell(&mut ctx, &mut lc, self.items.get(index).unwrap(), index)
                        }
                    }
                    ctx
                }),
            ]
            .map(|thread| thread.join().unwrap())
        })
        .unwrap();

        for thread_result in thread_results {
            combine(result, thread_result)
        }
    }

    pub fn len(&self) -> usize {
        self.items.len()
    }

    pub fn calc_index(&self, x: i32, y: i32) -> Option<usize> {
        if !self.x_indices.contains(&x) || !self.y_indices.contains(&y) {
            None
        } else {
            Some(
                (x - self.x_indices.start()) as usize
                    + (y - self.y_indices.start()) as usize * self.width,
            )
        }
    }

    pub fn get(&self, x: i32, y: i32) -> Option<&T> {
        self.items.get(self.calc_index(x, y)?)
    }

    pub fn get_mut(&mut self, x: i32, y: i32) -> Option<&mut T> {
        let ix = self.calc_index(x, y)?;
        self.items.get_mut(ix)
    }

    pub fn x_range(&self) -> RangeInclusive<i32> {
        self.x_indices.clone()
    }

    pub fn y_range(&self) -> RangeInclusive<i32> {
        self.y_indices.clone()
    }
}

impl<T: Copy + Into<char>> Debug for Grid<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("x = ")?;
        self.x_indices.fmt(f)?;
        f.write_str(", y = ")?;
        self.y_indices.fmt(f)?;
        f.write_str("\n\n")?;

        for y in self.y_indices.clone() {
            for x in self.x_indices.clone() {
                let item = self.get(x, y).unwrap();
                f.write_char((*item).into())?;
            }
            f.write_char('\n')?;
        }

        Ok(())
    }
}
