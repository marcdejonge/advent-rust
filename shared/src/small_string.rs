use smallvec::{Array, SmallVec};

#[derive(Debug, Default, Clone, Hash, PartialEq, Eq)]
pub struct SmallString<const S: usize>(SmallVec<[u8; S]>)
where
    [u8; S]: Array<Item = u8>;

impl<const S: usize> SmallString<S>
where
    [u8; S]: Array<Item = u8>,
{
    pub fn new() -> Self { Self::default() }

    pub fn from(s: &str) -> Self { SmallString(s.bytes().collect()) }

    pub fn len(&self) -> usize { self.0.len() }

    pub fn is_empty(&self) -> bool { self.0.is_empty() }

    pub fn repeat(self, c: u8, count: usize) -> Self {
        let mut result = self.0;
        for _ in 0..count {
            result.push(c);
        }
        SmallString(result)
    }

    pub fn close(self) -> Self {
        let mut result = self.0;
        result.push(b'A');
        SmallString(result)
    }

    pub fn iter(&self) -> impl Iterator<Item = &u8> { self.0.iter() }

    pub fn as_bytes(&self) -> &[u8] { &self.0 }
}
