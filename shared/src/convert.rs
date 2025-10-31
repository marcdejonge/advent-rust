pub trait Convert<From> {
    fn convert(value: From) -> Self;
}

impl<I, O> Convert<Vec<I>> for Vec<O>
where
    O: Convert<I>,
{
    fn convert(value: Vec<I>) -> Self {
        Vec::from_iter(value.into_iter().map(|v| Convert::convert(v)))
    }
}
