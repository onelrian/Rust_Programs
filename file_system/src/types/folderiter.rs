use crate::traits::filesystem::FileSystemItem;

use super::folder::Folder;

pub struct FolderIterator<'a> {
    folder: &'a Folder,
    index: usize,
}

impl<'a> Iterator for FolderIterator<'a> {
    type Item = &'a Box<dyn FileSystemItem>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.folder.get_items().len() {
            let item = &self.folder.get_items()[self.index];
            self.index += 1;
            Some(item)
        } else {
            None
        }
    }
}

impl Folder {
    pub fn iter(&self) -> FolderIterator {
        FolderIterator {
            folder: self,
            index: 0,
        }
    }
}
