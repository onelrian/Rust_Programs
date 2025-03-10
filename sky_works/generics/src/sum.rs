use std::ops::Add;

fn sum<T>(numbers: &[T]) -> T
where
    T: Add<Output = T> + Copy + Default,
{
    let mut total = T::default();
    for &number in numbers {
        total = total + number;
    }
    total
}