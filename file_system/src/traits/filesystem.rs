pub trait FileSystemItem {
    fn get_size(&self) -> usize;

    fn display(&self);
}
