pub(crate) struct Chunked<I, T>
where
    I: Iterator<Item = T>,
{
    iter: I,
    match_item: T,
}

pub(crate) trait ChunkedTrait {
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
        Chunked {
            iter: self,
            match_item: split_item,
        }
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
                None => {
                    return if sub_list.is_empty() {
                        None
                    } else {
                        Some(sub_list)
                    }
                }
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
pub(crate) struct ZipWithNext<I, T>
where
    I: Iterator<Item = T>,
    T: Copy,
{
    iter: I,
    last_result: Option<T>,
}

pub(crate) trait ZipWithNextTrait {
    fn zip_with_next<T>(self) -> ZipWithNext<Self, T>
    where
        Self: Iterator<Item = T> + Sized,
        T: Copy;
}

impl<I> ZipWithNextTrait for I
where
    I: Iterator,
{
    fn zip_with_next<T>(self) -> ZipWithNext<Self, T>
    where
        Self: Iterator<Item = T> + Sized,
        T: Copy,
    {
        ZipWithNext {
            iter: self,
            last_result: None,
        }
    }
}

impl<I, T> Iterator for ZipWithNext<I, T>
where
    I: Iterator<Item = T>,
    T: Copy,
{
    type Item = (T, T);

    fn next(&mut self) -> Option<Self::Item> {
        let mut next = self.iter.next()?;

        if self.last_result.is_none() {
            self.last_result = Some(next);
            next = self.iter.next()?;
        }

        let result = Some((self.last_result.unwrap(), next.clone()));
        self.last_result = Some(next);
        result
    }
}
