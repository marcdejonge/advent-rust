pub fn with_default<T, F>(build_function: F) -> T
where
    T: Default,
    F: FnOnce(&mut T),
{
    with(Default::default(), build_function)
}

pub fn with<T, F>(mut t: T, build_function: F) -> T
where
    F: FnOnce(&mut T),
{
    build_function(&mut t);
    t
}
