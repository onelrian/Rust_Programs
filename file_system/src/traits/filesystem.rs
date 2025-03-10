use std::fmt::Debug;

pub trait FileSystemItem: Debug {
    fn get_size(&self) -> usize;
    fn display(&self);
    fn display_with_indent(&self, depth: usize);
}
