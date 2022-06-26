use std::fmt::Display;

pub fn largest<T: PartialOrd + Copy>(list: &[T]) -> T
{
    let mut max = list[0];
    for &item in list{
        if item > max {
            max = item;
        }
    }
    max
}