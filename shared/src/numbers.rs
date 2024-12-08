use std::iter::Step;
use std::ops::RangeFrom;

#[derive(Copy, Clone, Debug)]
pub struct PositiveNumbersFrom<T>(pub T);

impl<T> IntoIterator for PositiveNumbersFrom<T>
where
    T: Step,
{
    type Item = T;
    type IntoIter = RangeFrom<T>;

    fn into_iter(self) -> Self::IntoIter { self.0.. }
}
