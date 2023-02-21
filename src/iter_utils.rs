pub(crate) struct Chunked<I, T> where I: Iterator<Item=T> {
    iter: I,
    match_item: T,
}

impl<I, T> Iterator for Chunked<I, T> where I: Iterator<Item=T>, T: Eq {
    type Item = Vec<T>;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        let mut sub_list: Vec<T> = Vec::new();

        loop {
            match self.iter.next() {
                None => return if sub_list.is_empty() { None } else { Some(sub_list) },
                Some(item) => if item == self.match_item {
                    return Some(sub_list);
                } else {
                    sub_list.push(item);
                },
            }
        }
    }
}

pub(crate) trait Chunkable {
    fn chunk_by<T>(self, split_item: T) -> Chunked<Self, T> where Self: Iterator<Item=T> + Sized;
}

impl<I> Chunkable for I where I: Iterator {
    fn chunk_by<T>(self, split_item: T) -> Chunked<Self, T> where Self: Iterator<Item=T> + Sized {
        Chunked { iter: self, match_item: split_item }
    }
}
