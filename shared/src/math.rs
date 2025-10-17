use std::ops::Sub;

pub fn greatest_common_divisor<N>(left: N, right: N) -> N
where
    N: Ord + Copy + Sub<Output = N>,
{
    match left.cmp(&right) {
        std::cmp::Ordering::Less => greatest_common_divisor(right, left),
        std::cmp::Ordering::Equal => left,
        std::cmp::Ordering::Greater => greatest_common_divisor(left - right, right),
    }
}
