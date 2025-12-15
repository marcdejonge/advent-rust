use itertools::Itertools;
use num_traits::{Num, Zero};
use std::fmt::{Debug, Display};
use std::iter::Sum;
use std::ops::{Index, Neg};

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct LinearEquation<const D: usize, T> {
    params: [T; D],
    result: T,
    param_length: usize,
}

impl<const D: usize, T> Debug for LinearEquation<D, T>
where
    T: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "LinearEquation [")?;
        for ix in 0..self.param_length {
            if ix > 0 {
                write!(f, ", ")?;
            }
            write!(f, "{}", self.params[ix])?;
        }
        write!(f, "] = {}", self.result)
    }
}

impl<const D: usize, T> LinearEquation<D, T> {
    pub fn new(parameters: &[T], result: T) -> Self
    where
        T: Zero + Copy,
    {
        assert!(
            parameters.len() <= D,
            "Cannot create an equation with more that {} parameters, but was {}",
            D,
            parameters.len()
        );

        let mut param_array = [T::zero(); D];
        param_array[0..parameters.len()].clone_from_slice(parameters);
        Self { params: param_array, result, param_length: parameters.len() }
    }
}

#[derive(Clone)]
pub struct LinearEquationSet<const D: usize, T> {
    equations: Vec<LinearEquation<D, T>>,
    param_length: usize,
}

impl<const D: usize, T> Debug for LinearEquationSet<D, T>
where
    T: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "LinearEquationSet: {{")?;
        for eq in &self.equations {
            writeln!(f, "  {:?}", eq)?;
        }
        writeln!(f, "}}")
    }
}

impl<const D: usize, T> LinearEquationSet<D, T> {
    pub fn new(equations: Vec<LinearEquation<D, T>>) -> Self {
        assert!(
            !equations.is_empty(),
            "Should have at least 1 equation to make a set"
        );

        let param_length = equations[0].param_length;
        assert!(
            equations.iter().all(|v| v.param_length == param_length),
            "All equations in a set should have the same number of parameters"
        );

        Self { equations, param_length }
    }

    /// Based on https://en.wikipedia.org/wiki/Gaussian_elimination
    /// Returns a list of indices, indicating which columns are free to set.
    /// For a set of equations that solve to a single solution, the free set should be empty.
    pub fn gaussian_elimination(&mut self) -> Vec<usize>
    where
        T: Num + Neg<Output = T> + Copy,
    {
        let mut free = Vec::new();
        let mut top_row_ix = 0;

        for column in 0..self.param_length {
            if let Some((row_ix, _)) = self
                .equations
                .iter()
                .enumerate()
                .skip(top_row_ix)
                .find(|(_, eq)| !eq.params[column].is_zero())
            {
                if top_row_ix != row_ix {
                    self.equations.swap(top_row_ix, row_ix);
                }

                // SAFETY: The top row can never become higher than the row index, so it's always safe
                let top_eq = unsafe { self.equations.get_unchecked_mut(top_row_ix) };

                // Now normalize the equation found, such that we are sure the column is a 1
                let div = top_eq.params[column];
                for pix in 0..self.param_length {
                    top_eq.params[pix] = top_eq.params[pix] / div;
                }
                top_eq.result = top_eq.result / div;

                let top_eq = *top_eq;

                // Now for all the other columns, subtract this until the columns becomes zero
                self.equations.iter_mut().enumerate().for_each(|(eq_ix, eq)| {
                    if eq_ix != top_row_ix {
                        let times = eq.params[column].neg();
                        for pix in 0..self.param_length {
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

    pub fn solve_many<G, I>(&self, generator: G) -> impl Iterator<Item = Vec<T>>
    where
        G: Fn(usize) -> I,
        I: Iterator<Item = T> + Clone,
        T: Num + Neg<Output = T> + Copy + Sum<T> + Display,
    {
        let mut eq_set = self.clone();
        let free = eq_set.gaussian_elimination();

        free.iter()
            .map(|&column| generator(column).map(move |v| (column, v)))
            .multi_cartesian_product()
            .map(move |free_vars| {
                eq_set
                    .equations
                    .iter()
                    .map(|equation| {
                        equation.result
                            - free_vars
                                .iter()
                                .map(|&(column, free_val)| equation.params[column] * free_val)
                                .sum::<T>()
                    })
                    .chain(free_vars.iter().map(|&(_, val)| val))
                    .collect()
            })
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
