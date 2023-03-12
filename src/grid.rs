use crossbeam::scope;

#[derive(Debug, Clone)]
pub struct Grid<T> {
    items: Vec<T>,
    height: usize,
    width: usize,
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
}
