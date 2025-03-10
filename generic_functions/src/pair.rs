#[derive(Debug)]
pub struct Pair<T, U> {
    part1: T,
    part2: U,
}

impl<T, U> Pair<T, U> {
    pub fn new(first: T, second: U) -> Self {
        Self {
            part1: first,
            part2: second,
        }
    }

    pub fn swap(self) -> Pair<U, T> {
        Pair {
            part1: self.part2,
            part2: self.part1,
        }
    }
}
