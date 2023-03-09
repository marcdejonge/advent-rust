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
    pub fn for_all_lines<C>(
        &self,
        context: &C,
        each_line: fn(&mut C),
        each_cell: fn(&mut C, &T, usize),
    ) -> [C; 4]
    where
        C: Clone + Sync + Send,
        T: Sync + Send,
    {
        scope(|s| {
            let horizontal_thread = s.spawn(|_| {
                let mut ctx = context.clone();
                for y in 0..self.height {
                    each_line(&mut ctx);
                    for index in (y * self.width)..((y + 1) * self.width) {
                        each_cell(&mut ctx, self.items.get(index).unwrap(), index)
                    }
                }
                ctx
            });

            let horizontal_rev_thread = s.spawn(|_| {
                let mut ctx = context.clone();
                for y in 0..self.height {
                    each_line(&mut ctx);
                    for index in ((y * self.width)..((y + 1) * self.width)).rev() {
                        each_cell(&mut ctx, self.items.get(index).unwrap(), index)
                    }
                }
                ctx
            });

            let vertical_thread = s.spawn(|_| {
                let mut ctx = context.clone();
                for x in 0..self.width {
                    each_line(&mut ctx);
                    for index in (x..(self.height * self.width)).step_by(self.width) {
                        each_cell(&mut ctx, self.items.get(index).unwrap(), index)
                    }
                }
                ctx
            });

            let vertical_rev_thread = s.spawn(|_| {
                let mut ctx = context.clone();
                for x in 0..self.width {
                    each_line(&mut ctx);
                    for index in (x..(self.height * self.width)).step_by(self.width).rev() {
                        each_cell(&mut ctx, self.items.get(index).unwrap(), index)
                    }
                }
                ctx
            });

            [
                horizontal_thread.join().unwrap(),
                horizontal_rev_thread.join().unwrap(),
                vertical_thread.join().unwrap(),
                vertical_rev_thread.join().unwrap(),
            ]
        })
        .unwrap()
    }

    pub fn len(&self) -> usize {
        self.items.len()
    }
}
