use std::array::from_fn;
use std::cmp::Ordering;
use std::fmt::Debug;
use std::hash::Hash;
use std::iter::Sum;
use std::mem;

use fxhash::FxHashMap;

pub trait IteratorUtils: Iterator {
    fn chunk_by(self, split_item: Self::Item) -> impl Iterator<Item = Vec<Self::Item>>
    where
        Self: Sized,
        Self::Item: Eq,
    {
        Chunked { iter: self, match_item: split_item }
    }

    fn repeat(self) -> impl Iterator<Item = Self::Item>
    where
        Self: Sized + Clone,
    {
        RepeatingIterator { current: self.clone(), source: self }
    }

    fn zip_with_next(self) -> impl Iterator<Item = (Self::Item, Self::Item)>
    where
        Self: Sized,
        Self::Item: Clone,
    {
        ZipWithNext { iter: self, last_result: None }
    }

    fn counts_fx(self) -> FxHashMap<Self::Item, usize>
    where
        Self: Sized,
        Self::Item: Eq + Hash,
    {
        let mut counts = FxHashMap::default();
        self.for_each(|item| *counts.entry(item).or_insert(0) += 1);
        counts
    }

    #[inline]
    fn take_n<const N: usize>(mut self) -> [Self::Item; N]
    where
        Self: Sized,
        Self::Item: Default + Copy,
    {
        let mut result: [Self::Item; N] = [Default::default(); N];
        for ix in 0..N {
            result[ix] = self.next().unwrap();
        }
        result
    }

    fn max_n<const N: usize>(self) -> [Self::Item; N]
    where
        Self: Sized,
        Self::Item: Default + Copy + PartialOrd + Debug,
    {
        let mut result: [Self::Item; N] = [Default::default(); N];

        for item in self {
            if item > result[N - 1] {
                result[N - 1] = item;
                for ix in (0..(N - 1)).rev() {
                    if item > result[ix] {
                        result[ix + 1] = result[ix];
                        result[ix] = item;
                    } else {
                        break;
                    }
                }
            }
        }

        result
    }

    fn single(mut self) -> Option<Self::Item>
    where
        Self: Sized,
    {
        self.next().filter(|_| self.next().is_none())
    }

    fn combinations<const D: usize>(self) -> impl Iterator<Item = [Self::Item; D]>
    where
        Self: Sized + Clone,
        Self::Item: Clone,
    {
        let mut result =
            AllSetsIterator { iters: from_fn(|_| self.clone()), current_values: from_fn(|_| None) };
        for ix in 0..(D - 1) {
            result.current_values[ix] = result.iters[ix].next();
            result.iters[ix + 1] = result.iters[ix].clone();
        }
        result
    }

    fn top<const D: usize>(
        self,
        sort_with: impl Fn(&Self::Item, &Self::Item) -> Ordering,
    ) -> Option<[Self::Item; D]>
    where
        Self: Sized,
        Self::Item: Clone,
    {
        let mut top: [Option<Self::Item>; D] = from_fn(|_| None);
        self.for_each(|item| {
            for test_ix in 0..D {
                if let Some(stored) = top[test_ix].clone() {
                    if sort_with(&item, &stored) == Ordering::Greater {
                        for move_ix in ((test_ix + 1)..D).rev() {
                            top[move_ix] = top[move_ix - 1].clone();
                        }
                        top[test_ix] = Some(item);
                        break;
                    }
                } else {
                    top[test_ix] = Some(item);
                    break;
                }
            }
        });

        if top.iter().any(|option| option.is_none()) {
            None
        } else {
            unsafe { Some(top.map(|option| option.unwrap_unchecked())) }
        }
    }

    fn find_cyclic_result_at<S, T>(mut self, target_index: usize) -> Option<T>
    where
        Self: Sized + Iterator<Item = (S, T)>,
        S: Eq + Hash,
        T: PartialEq + Clone + Debug,
    {
        let mut results = Vec::<T>::with_capacity(512);
        let mut states = FxHashMap::<S, usize>::default();

        loop {
            let (state, result) = self.next()?;
            results.push(result);

            if let Some(last_length) = states.get(&state) {
                // Found same state, we can determine the location of previous
                let cycle_size = results.len() - last_length;
                let steps_needed = target_index - results.len();
                let cycles = steps_needed / cycle_size;
                let index = target_index - (cycles * cycle_size) - cycle_size;
                return Some(results[index - 1].clone());
            }

            states.insert(state, results.len());
        }
    }
}

impl<T> IteratorUtils for T where T: Iterator + ?Sized {}

struct Chunked<I, T>
where
    I: Iterator<Item = T>,
{
    iter: I,
    match_item: T,
}

impl<I, T> Iterator for Chunked<I, T>
where
    I: Iterator<Item = T>,
    T: Eq,
{
    type Item = Vec<T>;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        let mut sub_list: Vec<T> = Vec::new();

        loop {
            match self.iter.next() {
                None => return if sub_list.is_empty() { None } else { Some(sub_list) },
                Some(item) => {
                    if item == self.match_item {
                        return Some(sub_list);
                    } else {
                        sub_list.push(item);
                    }
                }
            }
        }
    }
}

// implementation to generate .zip_with_next() function for all iterators
struct ZipWithNext<I, T>
where
    I: Iterator<Item = T>,
{
    iter: I,
    last_result: Option<T>,
}

impl<I, T> Iterator for ZipWithNext<I, T>
where
    I: Iterator<Item = T>,
    T: Clone,
{
    type Item = (T, T);

    fn next(&mut self) -> Option<Self::Item> {
        let mut next = self.iter.next()?;

        if self.last_result.is_none() {
            self.last_result = Some(next);
            next = self.iter.next()?;
        }

        Some((
            mem::replace(&mut self.last_result, Some(next.clone())).unwrap(),
            next,
        ))
    }
}

struct RepeatingIterator<I, T>
where
    I: Iterator<Item = T> + Clone,
{
    source: I,
    current: I,
}

impl<I, T> Iterator for RepeatingIterator<I, T>
where
    I: Iterator<Item = T> + Clone + Sized,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let next = self.current.next();
        if next.is_some() {
            next
        } else {
            self.current = self.source.clone();
            self.current.next()
        }
    }
}

#[cfg(test)]
mod zip_with_next_tests {
    use super::IteratorUtils;

    #[test]
    fn test_normal_behavior() {
        assert_eq!(
            [1, 2, 3, 4].iter().zip_with_next().collect::<Vec<_>>(),
            vec![(&1, &2), (&2, &3), (&3, &4)]
        )
    }

    #[test]
    fn empty_vector() {
        assert_eq!(
            Vec::<i32>::new().iter().zip_with_next().collect::<Vec<_>>(),
            vec![]
        )
    }
    #[test]
    fn single_item() { assert_eq!([1].iter().zip_with_next().collect::<Vec<_>>(), vec![]) }
}

#[test]
fn test_max_n() {
    assert_eq!((0..100).max_n(), [99, 98, 97]);
    assert_eq!((0..100).step_by(5).max_n(), [95, 90]);
    assert_eq!((0..100).step_by(5).rev().max_n(), [95, 90, 85, 80]);
}

#[test]
fn check_single() {
    assert_eq!(None, [1, 2, 3].iter().single());
    assert_eq!(None, [1, 2].iter().single());
    assert_eq!(Some(&1), [1].iter().single());
    assert_eq!(None::<&i32>, [].iter().single());
}

struct AllSetsIterator<const D: usize, I, T> {
    iters: [I; D],
    current_values: [Option<T>; D],
}

impl<const D: usize, I, T> AllSetsIterator<D, I, T>
where
    I: Iterator<Item = T> + Clone,
    T: Clone,
{
    fn get_next_item(&mut self, from: usize) -> bool {
        if let Some(value) = self.iters[from].next() {
            self.current_values[from] = Some(value);
            true
        } else if from > 0 {
            if self.get_next_item(from - 1) {
                self.iters[from] = self.iters[from - 1].clone();
                self.get_next_item(from)
            } else {
                false
            }
        } else {
            false
        }
    }
}

impl<const D: usize, I, T> Iterator for AllSetsIterator<D, I, T>
where
    I: Iterator<Item = T> + Clone,
    T: Clone,
{
    type Item = [T; D];

    fn next(&mut self) -> Option<Self::Item> {
        if self.get_next_item(D - 1) {
            Some(self.current_values.clone().map(|x| x.unwrap()))
        } else {
            None
        }
    }
}

#[test]
fn check_all_pairs() {
    assert_eq!(
        vec![[1, 2], [1, 3], [1, 4], [2, 3], [2, 4], [3, 4]],
        [1, 2, 3, 4].into_iter().combinations().collect::<Vec<_>>()
    );
}

#[test]
fn check_all_triples() {
    assert_eq!(
        vec![[1, 2, 3], [1, 2, 4], [1, 3, 4], [2, 3, 4]],
        [1, 2, 3, 4].into_iter().combinations().collect::<Vec<_>>()
    );
}

#[test]
fn test_getting_top_results() {
    assert_eq!(Some([9, 8, 7]), (1..10).top(usize::cmp));
    assert_eq!(None, (1..3).top::<3>(usize::cmp));
}

pub trait CountIf<F> {
    fn count_if(self, predicate: F) -> usize;
}

impl<F, L> CountIf<F> for L
where
    L: IntoIterator,
    F: FnMut(&L::Item) -> bool,
{
    #[inline]
    fn count_if(self, predicate: F) -> usize { self.into_iter().filter(predicate).count() }
}

pub trait SumWith<O, F> {
    fn sum_with(self, mapping: F) -> O;
}

impl<I, O, L, F> SumWith<O, F> for L
where
    L: IntoIterator<Item = I>,
    F: FnMut(I) -> O,
    O: Sum,
{
    #[inline]
    fn sum_with(self, mapping: F) -> O { self.into_iter().map(mapping).sum() }
}
