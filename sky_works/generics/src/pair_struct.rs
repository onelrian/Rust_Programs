struct Pair<T> {
    first: T,
    second: T,
}

impl<T> Pair<T> {
    fn swap(&mut self) {
        std::mem::swap(&mut self.first, &mut self.second);
    }
}