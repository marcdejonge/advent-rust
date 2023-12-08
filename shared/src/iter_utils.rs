use std::fmt::Debug;
use std::mem;

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
