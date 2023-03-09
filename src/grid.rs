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
        context: &mut C,
        each_line: fn(&mut C),
        each_cell: fn(&mut C, &T, usize),
    ) {
        for y in 0..self.height {
            each_line(context);
            for index in (y * self.height)..((y + 1) * self.height) {
                each_cell(context, self.items.get(index).unwrap(), index)
            }
            each_line(context);
            for index in ((y * self.height)..((y + 1) * self.height)).rev() {
                each_cell(context, self.items.get(index).unwrap(), index)
            }
        }

        for x in 0..self.width {
            each_line(context);
            for index in (x..(self.height * self.width)).step_by(self.width) {
                each_cell(context, self.items.get(index).unwrap(), index)
            }
            each_line(context);
            for index in (x..(self.height * self.width)).step_by(self.width).rev() {
                each_cell(context, self.items.get(index).unwrap(), index)
            }
        }
    }

    pub fn len(&self) -> usize {
        self.items.len()
    }
}
