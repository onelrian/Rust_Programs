use crate::traits::filesystem::FileSystemItem;

#[derive()]
pub struct Folder {
    name: String,
    items: Vec<Box<dyn FileSystemItem>>,
}

impl Drop for Folder {
    fn drop(&mut self) {
        println!("Folder: {} is being deleted",self.name)
    }
}

impl Folder {
    pub fn new(name: String) -> Self {
        Self {
            name,
            items: Vec::new(),
        }
    }

    pub fn add_item(&mut self, item: Box<dyn FileSystemItem>) {
        self.items.push(item);
    }

    pub fn get_items(&self) -> &Vec<Box<dyn FileSystemItem>> {
        &self.items
    }
}
impl FileSystemItem for Folder {
    fn get_size(&self) -> usize {
        self.items.iter().map(|item| item.get_size()).sum()
    }

    fn display(&self) {
        println!("Folder: {}", self.name);
        for item in &self.items {
            item.display();
        }
    }
}
