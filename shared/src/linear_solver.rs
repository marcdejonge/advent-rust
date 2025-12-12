use num_traits::{Num, Zero};
use std::fmt::Debug;
use std::ops::{Index, Neg};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LinearEquation<const D: usize, T> {
    params: [T; D],
    result: T,
}

impl<const D: usize, T> LinearEquation<D, T> {
    pub fn new(parameters: &[T], result: T) -> Self
    where
        T: Zero + Copy,
    {
        let mut param_array = [T::zero(); D];
        param_array[0..parameters.len().min(D)].clone_from_slice(parameters);
        Self { params: param_array, result }
    }
}

#[derive(Debug)]
pub struct LinearEquationSet<const D: usize, T> {
    equations: Vec<LinearEquation<D, T>>,
}

impl<const D: usize, T> LinearEquationSet<D, T> {
    pub fn new(equations: Vec<LinearEquation<D, T>>) -> Self { Self { equations } }

    /// Based on https://en.wikipedia.org/wiki/Gaussian_elimination
    /// Returns a list of indices, indicating which columns are free to set.
    /// For a set of equations that solve to a single solution, the free set should be empty.
    pub fn gaussian_elimination(&mut self) -> Vec<usize>
    where
        T: Num + Neg<Output = T> + Copy + Debug,
    {
        let mut free = Vec::new();
        let mut top_row_ix = 0;

        for column in 0..D {
            if let Some((row_ix, _)) = self
                .equations
                .iter()
                .enumerate()
                .skip(top_row_ix)
                .find(|(_, eq)| eq.params[column] != T::zero())
            {
                if top_row_ix != row_ix {
                    self.equations.swap(top_row_ix, row_ix);
                }

                // SAFETY: The top row can never become higher than the row index, so it's always safe
                let top_eq = unsafe { self.equations.get_unchecked_mut(top_row_ix) };

                // Now normalize the equation found, such that we are sure the column is a 1
                let div = top_eq.params[column];
                for pix in column..D {
                    top_eq.params[pix] = top_eq.params[pix] / div;
                }
                top_eq.result = top_eq.result / div;

                let top_eq = *top_eq;

                // Now for all the other columns, subtract this until the columns becomes zero
                self.equations.iter_mut().enumerate().for_each(|(eq_ix, eq)| {
                    if eq_ix != top_row_ix {
                        let times = eq.params[column].neg();
                        for pix in column..D {
                            eq.params[pix] = eq.params[pix] + times * top_eq.params[pix]
                        }
                        eq.result = eq.result + times * top_eq.result;
                    }
                });

                // Increment the top row for the next row
                top_row_ix += 1;
            } else {
                // There were no equations (left) on this column, so this one is free and we just continue
                free.push(column);
            }
        }

        free
    }
}

impl<const D: usize, T> Index<usize> for LinearEquationSet<D, T> {
    type Output = LinearEquation<D, T>;

    fn index(&self, index: usize) -> &Self::Output { self.equations.get(index).unwrap() }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_gaussian() {
        let mut eq_set = LinearEquationSet::<3, f64>::new(vec![
            LinearEquation::new(&[2., 1., -1.], 8.),
            LinearEquation::new(&[-3., -1., 2.], -11.),
            LinearEquation::new(&[-2., 1., 2.], -3.),
        ]);
        let free = eq_set.gaussian_elimination();
        assert!(free.is_empty());
        assert_eq!(LinearEquation::new(&[1., 0., 0.], 2.), eq_set[0]);
        assert_eq!(LinearEquation::new(&[0., 1., 0.], 3.), eq_set[1]);
        assert_eq!(LinearEquation::new(&[0., 0., 1.], -1.), eq_set[2]);
    }
}
