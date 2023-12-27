use std::array::from_fn;
use std::cmp::Ordering;
use std::fmt::Debug;
use std::hash::Hash;
use std::mem;

use fxhash::FxHashMap;

pub struct Chunked<I, T>
where
    I: Iterator<Item = T>,
{
    iter: I,
    match_item: T,
}

pub trait ChunkedTrait {
    fn chunk_by<T>(self, split_item: T) -> Chunked<Self, T>
    where
        Self: Iterator<Item = T> + Sized;
}

impl<I> ChunkedTrait for I
where
    I: Iterator,
{
    fn chunk_by<T>(self, split_item: T) -> Chunked<Self, T>
    where
        Self: Iterator<Item = T> + Sized,
    {
        Chunked { iter: self, match_item: split_item }
    }
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
pub struct ZipWithNext<I, T>
where
    I: Iterator<Item = T>,
{
    iter: I,
    last_result: Option<T>,
}

pub trait ZipWithNextTrait {
    fn zip_with_next<T>(self) -> ZipWithNext<Self, T>
    where
        Self: Iterator<Item = T> + Sized;
}

impl<I> ZipWithNextTrait for I
where
    I: Iterator,
{
    fn zip_with_next<T>(self) -> ZipWithNext<Self, T>
    where
        Self: Iterator<Item = T>,
    {
        ZipWithNext { iter: self, last_result: None }
    }
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

pub struct RepeatingIterator<I, T>
where
    I: Iterator<Item = T> + Clone,
{
    source: I,
    current: I,
}

pub trait RepeatingIteratorTrait {
    fn repeat<T>(self) -> RepeatingIterator<Self, T>
    where
        Self: Iterator<Item = T> + Clone + Sized;
}

impl<I> RepeatingIteratorTrait for I
where
    I: Iterator + Clone + Sized,
{
    fn repeat<T>(self) -> RepeatingIterator<Self, T>
    where
        I: Iterator<Item = T>,
    {
        RepeatingIterator { current: self.clone(), source: self }
    }
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
    use super::*;

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

pub fn max_n<const N: usize, T>(it: impl Iterator<Item = T>) -> [T; N]
where
    T: Default + Copy + PartialOrd + Debug,
{
    let mut result: [T; N] = [Default::default(); N];

    for item in it {
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

#[test]
fn test_max_n() {
    assert_eq!(max_n(0..100), [99, 98, 97]);
    assert_eq!(max_n((0..100).step_by(5)), [95, 90]);
    assert_eq!(max_n((0..100).step_by(5).rev()), [95, 90, 85, 80]);
}

pub trait DetectingCycleTrait<T> {
    fn find_cyclic_result_at(self, target_index: usize) -> Option<T>;
}

impl<I, S, T> DetectingCycleTrait<T> for I
where
    I: Iterator<Item = (S, T)> + Sized,
    S: Eq + Hash,
    T: PartialEq + Clone + Debug,
{
    fn find_cyclic_result_at(mut self, target_index: usize) -> Option<T> {
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

pub trait SingleTrait<T> {
    fn single(self) -> Option<T>;
}

impl<I, T> SingleTrait<T> for I
where
    I: Iterator<Item = T>,
{
    fn single(mut self) -> Option<T> { self.next().filter(|_| self.next().is_none()) }
}

#[test]
fn check_single() {
    assert_eq!(None, [1, 2, 3].iter().single());
    assert_eq!(None, [1, 2].iter().single());
    assert_eq!(Some(&1), [1].iter().single());
    assert_eq!(None::<&i32>, [].iter().single());
}

pub struct AllSetsIterator<const D: usize, I, T> {
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

pub trait AllSetsTrait<T>
where
    Self: Sized,
{
    fn combinations<const D: usize>(self) -> AllSetsIterator<D, Self, T>;
}

impl<I, T> AllSetsTrait<T> for I
where
    I: Iterator<Item = T> + Clone,
    T: Clone,
{
    fn combinations<const D: usize>(self) -> AllSetsIterator<D, Self, T> {
        let mut result =
            AllSetsIterator { iters: from_fn(|_| self.clone()), current_values: from_fn(|_| None) };
        for ix in 0..(D - 1) {
            result.current_values[ix] = result.iters[ix].next();
            result.iters[ix + 1] = result.iters[ix].clone();
        }
        result
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

pub trait TopSelectTrait<T> {
    fn top<const D: usize>(self, sort_with: impl Fn(&T, &T) -> Ordering) -> Option<[T; D]>;
}

impl<I, T> TopSelectTrait<T> for I
where
    I: Iterator<Item = T>,
    T: Copy,
{
    fn top<const D: usize>(self, sort_with: impl Fn(&T, &T) -> Ordering) -> Option<[T; D]> {
        let mut top: [Option<T>; D] = from_fn(|_| None);
        self.for_each(|item| {
            for test_ix in 0..D {
                if let Some(stored) = top[test_ix] {
                    if sort_with(&item, &stored) == Ordering::Greater {
                        for move_ix in ((test_ix + 1)..D).rev() {
                            top[move_ix] = top[move_ix - 1];
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
}

#[test]
fn test_getting_top_results() {
    assert_eq!(Some([9, 8, 7]), (1..10).top(usize::cmp));
    assert_eq!(None, (1..3).top::<3>(usize::cmp));
}
